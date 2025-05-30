

#![no_main]

use rollup_config::ROLLUP_NAMESPACE_RAW;
use stf::runtime::Runtime;
use stf::AppVerifier;
use celestia_adapter::types::Namespace;
use celestia_adapter::verifier::CelestiaVerifier;
use sov_modules_api::default_context::ZkDefaultContext;
use sov_modules_stf_template::AppTemplate;
use risc0_adapter::guest::Risc0Guest;
use sov_state::ZkStorage;

// The rollup stores its data in the namespace b"sov-test" on Celestia
const ROLLUP_NAMESPACE: Namespace = Namespace::const_v0(ROLLUP_NAMESPACE_RAW);

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let guest = Risc0Guest::new();
    let storage = ZkStorage::new();
    let app: AppTemplate<ZkDefaultContext, _, _, Runtime<_, _>> = AppTemplate::new(storage);

    let mut stf_verifier = AppVerifier::new(
        app,
        CelestiaVerifier {
            rollup_namespace: ROLLUP_NAMESPACE,
        },
    );
    stf_verifier
        .run_block(guest)
        .expect("Prover must be honest");
}
