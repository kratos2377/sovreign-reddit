#[cfg(feature = "experimental")]
use std::str::FromStr;

use anyhow::Context as _;
use stf::runtime::Runtime;
#[cfg(feature = "experimental")]
use secp256k1::SecretKey;
#[cfg(feature = "experimental")]
use sov_cli::wallet_state::PrivateKeyAndAddress;
use sov_db::ledger_db::LedgerDB;
#[cfg(feature = "experimental")]
use sov_ethereum::experimental::EthRpcConfig;
#[cfg(feature = "experimental")]
use sov_ethereum::GasPriceOracleConfig;
use sov_modules_api::default_context::DefaultContext;
#[cfg(feature = "experimental")]
use sov_modules_api::default_signature::private_key::DefaultPrivateKey;
use sov_modules_api::Spec;
use sov_modules_stf_template::{SequencerOutcome, TxEffect};
use sov_rollup_interface::da::DaSpec;
use sov_rollup_interface::services::batch_builder::BatchBuilder;
use sov_rollup_interface::services::da::DaService;
use sov_sequencer::batch_builder::FiFoStrictBatchBuilder;
use sov_sequencer::get_sequencer_rpc;
use sov_state::ProverStorage;

pub(crate) fn create_rpc_methods<Da: DaService + Clone>(
    storage: &<DefaultContext as Spec>::Storage,
    ledger_db: &LedgerDB,
    da_service: Da,
) -> Result<jsonrpsee::RpcModule<()>, anyhow::Error> {
    let batch_builder = create_batch_builder::<<Da as DaService>::Spec>(storage.clone());

    let mut methods = stf::runtime::get_rpc_methods::<DefaultContext, <Da as DaService>::Spec>(
        storage.clone(),
    );

    methods.merge(
        sov_ledger_rpc::server::rpc_module::<
            LedgerDB,
            SequencerOutcome<<<Da as DaService>::Spec as DaSpec>::Address>,
            TxEffect,
        >(ledger_db.clone())?
        .remove_context(),
    )?;

    register_sequencer(da_service.clone(), batch_builder, &mut methods)?;

  

    Ok(methods)
}

fn register_sequencer<Da: DaService, B: BatchBuilder + Send + Sync + 'static>(
    da_service: Da,
    batch_builder: B,
    methods: &mut jsonrpsee::RpcModule<()>,
) -> Result<(), anyhow::Error> {
    let sequencer_rpc = get_sequencer_rpc(batch_builder, da_service);
    methods
        .merge(sequencer_rpc)
        .context("Failed to merge Txs RPC modules")
}

fn create_batch_builder<Da: DaSpec>(
    storage: ProverStorage<sov_state::DefaultStorageSpec>,
) -> FiFoStrictBatchBuilder<Runtime<DefaultContext, Da>, DefaultContext> {
    let batch_size_bytes = 1024 * 100; // 100 KB
    FiFoStrictBatchBuilder::new(
        batch_size_bytes,
        u32::MAX as usize,
        Runtime::default(),
        storage,
    )
}
