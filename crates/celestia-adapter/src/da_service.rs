use async_trait::async_trait;
use celestia_rpc::prelude::*;
use celestia_types::blob::{Blob as JsonBlob, Commitment, SubmitOptions};
use celestia_types::consts::appconsts::{
    CONTINUATION_SPARSE_SHARE_CONTENT_SIZE, FIRST_SPARSE_SHARE_CONTENT_SIZE, SHARE_SIZE,
};
use celestia_types::nmt::Namespace;
use jsonrpsee::http_client::{HeaderMap, HttpClient};
use sov_rollup_interface::da::CountedBufReader;
use sov_rollup_interface::services::da::DaService;
use tracing::{debug, info, instrument, trace};

use crate::shares::Blob;
use crate::types::FilteredCelestiaBlock;
use crate::utils::BoxError;
use crate::verifier::proofs::{CompletenessProof, CorrectnessProof};
use crate::verifier::{CelestiaSpec, CelestiaVerifier, RollupParams, PFB_NAMESPACE};
use crate::BlobWithSender;

// Approximate value, just to make it work.
const GAS_PER_BYTE: usize = 20;
const GAS_PRICE: usize = 1;

#[derive(Debug, Clone)]
pub struct CelestiaService {
    client: HttpClient,
    rollup_namespace: Namespace,
}

impl CelestiaService {
    pub fn with_client(client: HttpClient, nid: Namespace) -> Self {
        Self {
            client,
            rollup_namespace: nid,
        }
    }
}

/// Runtime configuration for the DA service
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DaServiceConfig {
    /// The jwt used to authenticate with the Celestia rpc server
    pub celestia_rpc_auth_token: String,
    /// The address of the Celestia rpc server
    #[serde(default = "default_rpc_addr")]
    pub celestia_rpc_address: String,
    /// The maximum size of a Celestia RPC response, in bytes
    #[serde(default = "default_max_response_size")]
    pub max_celestia_response_body_size: u32,
    /// The timeout for a Celestia RPC request, in seconds
    #[serde(default = "default_request_timeout_seconds")]
    pub celestia_rpc_timeout_seconds: u64,
}

fn default_rpc_addr() -> String {
    "http://localhost:11111/".into()
}

fn default_max_response_size() -> u32 {
    1024 * 1024 * 100 // 100 MB
}

const fn default_request_timeout_seconds() -> u64 {
    60
}

impl CelestiaService {
    pub async fn new(config: DaServiceConfig, chain_params: RollupParams) -> Self {
        let client = {
            let mut headers = HeaderMap::new();
            headers.insert(
                "Authorization",
                format!("Bearer {}", config.celestia_rpc_auth_token)
                    .parse()
                    .unwrap(),
            );

            jsonrpsee::http_client::HttpClientBuilder::default()
                .set_headers(headers)
                .max_request_size(config.max_celestia_response_body_size)
                .request_timeout(std::time::Duration::from_secs(
                    config.celestia_rpc_timeout_seconds,
                ))
                .build(&config.celestia_rpc_address)
        }
        .expect("Client initialization is valid");

        Self::with_client(client, chain_params.namespace)
    }
}

#[async_trait]
impl DaService for CelestiaService {
    type Spec = CelestiaSpec;

    type Verifier = CelestiaVerifier;

    type FilteredBlock = FilteredCelestiaBlock;

    type Error = BoxError;

    #[instrument(skip(self), err)]
    async fn get_finalized_at(&self, height: u64) -> Result<Self::FilteredBlock, Self::Error> {
        let client = self.client.clone();
        let rollup_namespace = self.rollup_namespace;

        // Fetch the header and relevant shares via RPC
        debug!("Fetching header");
        let header = client.header_get_by_height(height).await?;
        trace!(header_result = ?header);

        // Fetch the rollup namespace shares, etx data and extended data square
        debug!("Fetching rollup data...");
        let rollup_rows_future = client.share_get_shares_by_namespace(&header, rollup_namespace);
        let etx_rows_future = client.share_get_shares_by_namespace(&header, PFB_NAMESPACE);
        let data_square_future = client.share_get_eds(&header);

        let (rollup_rows, etx_rows, data_square) =
            tokio::try_join!(rollup_rows_future, etx_rows_future, data_square_future)?;

        FilteredCelestiaBlock::new(
            self.rollup_namespace,
            header,
            rollup_rows,
            etx_rows,
            data_square,
        )
    }

    async fn get_block_at(&self, height: u64) -> Result<Self::FilteredBlock, Self::Error> {
        self.get_finalized_at(height).await
    }

    fn extract_relevant_blobs(
        &self,
        block: &Self::FilteredBlock,
    ) -> Vec<<Self::Spec as sov_rollup_interface::da::DaSpec>::BlobTransaction> {
        let mut output = Vec::new();
        for blob_ref in block.rollup_data.blobs() {
            let commitment = Commitment::from_shares(self.rollup_namespace, blob_ref.0)
                .expect("blob must be valid");
            info!("Blob: {:?}", commitment);
            let sender = block
                .relevant_pfbs
                .get(&commitment.0[..])
                .expect("blob must be relevant")
                .0
                .signer
                .clone();

            let blob: Blob = blob_ref.into();

            let blob_tx = BlobWithSender {
                blob: CountedBufReader::new(blob.into_iter()),
                sender: sender.parse().expect("Incorrect sender address"),
                hash: commitment.0,
            };

            output.push(blob_tx)
        }
        output
    }

    async fn get_extraction_proof(
        &self,
        block: &Self::FilteredBlock,
        blobs: &[<Self::Spec as sov_rollup_interface::da::DaSpec>::BlobTransaction],
    ) -> (
        <Self::Spec as sov_rollup_interface::da::DaSpec>::InclusionMultiProof,
        <Self::Spec as sov_rollup_interface::da::DaSpec>::CompletenessProof,
    ) {
        let etx_proofs = CorrectnessProof::for_block(block, blobs);
        let rollup_row_proofs = CompletenessProof::from_filtered_block(block);

        (etx_proofs.0, rollup_row_proofs.0)
    }

    #[instrument(skip_all, err)]
    async fn send_transaction(&self, blob: &[u8]) -> Result<(), Self::Error> {
        debug!("Sending {} bytes of raw data to Celestia.", blob.len());

        let gas_limit = get_gas_limit_for_bytes(blob.len()) as u64;
        let fee = gas_limit * GAS_PRICE as u64;

        let blob = JsonBlob::new(self.rollup_namespace, blob.to_vec())?;
        info!("Submiting: {:?}", blob.commitment);

        let height = self
            .client
            .blob_submit(
                &[blob],
                SubmitOptions {
                    fee: Some(fee),
                    gas_limit: Some(gas_limit),
                },
            )
            .await?;
        info!(
            "Blob has been submitted to Celestia. block-height={}",
            height,
        );
        Ok(())
    }
}

// https://docs.celestia.org/learn/submit-data/#fees-and-gas-limits
fn get_gas_limit_for_bytes(n: usize) -> usize {
    let fixed_cost = 75000;

    let continuation_shares_needed =
        n.saturating_sub(FIRST_SPARSE_SHARE_CONTENT_SIZE) / CONTINUATION_SPARSE_SHARE_CONTENT_SIZE;
    let shares_needed = 1 + continuation_shares_needed + 1; // add one extra, pessimistic

    fixed_cost + shares_needed * SHARE_SIZE * GAS_PER_BYTE
}