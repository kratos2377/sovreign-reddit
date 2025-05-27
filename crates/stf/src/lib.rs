#[cfg(feature = "native")]
pub mod cli;
#[cfg(feature = "native")]
pub mod genesis_config;
mod hooks_impl;
pub mod runtime;

use sov_modules_stf_template::AppTemplate;
use sov_rollup_interface::da::DaVerifier;
use sov_stf_runner::verifier::StateTransitionVerifier;

/// Alias for StateTransitionVerifier.
pub type AppVerifier<DA, Vm, ZkContext, RT> =
    StateTransitionVerifier<AppTemplate<ZkContext, <DA as DaVerifier>::Spec, Vm, RT>, DA, Vm>;
