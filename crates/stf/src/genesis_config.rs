//! While the `GenesisConfig` type for `Rollup` is generated from the underlying runtime through a macro,
//! specific module configurations are obtained from files. This code is responsible for the logic
//! that transforms module genesis data into Rollup genesis data.

use std::convert::AsRef;
use std::path::{Path, PathBuf};

use anyhow::bail;
use sov_chain_state::ChainStateConfig;
pub use sov_modules_api::default_context::DefaultContext;
use sov_modules_api::Context;
use sov_rollup_interface::da::DaSpec;
use sov_sequencer_registry::SequencerConfig;
pub use sov_state::config::Config as StorageConfig;
use sov_stf_runner::read_json_file;
use sov_value_setter::ValueSetterConfig;
use reddit::RedditConfig;

/// Creates config for a rollup with some default settings, the config is used in demos and tests.
use crate::runtime::GenesisConfig;

/// Paths pointing to genesis files.
pub struct GenesisPaths<P: AsRef<Path>> {
    /// Sequencer Registry genesis path.
    pub sequencer_genesis_path: P,
    /// Value Setter genesis path.
    pub value_setter_genesis_path: P,
    /// Reddit genesis path.
    pub reddit_genesis_path: P,
    /// Chain State genesis path.
    pub chain_state_genesis_path: P,
}

impl GenesisPaths<PathBuf> {
    /// Creates a new [`GenesisPaths`] from the files contained in the given
    /// directory.
    ///
    /// Take a look at the contents of the `test_data` directory to see the
    /// expected files.
    pub fn from_dir(dir: impl AsRef<Path>) -> Self {
        Self {
         
            sequencer_genesis_path: dir.as_ref().join("sequencer_registry.json"),
            value_setter_genesis_path: dir.as_ref().join("value_setter.json"),
            reddit_genesis_path: dir.as_ref().join("accounts.json"),
            chain_state_genesis_path: dir.as_ref().join("chain_state.json"),

        }
    }
}

/// Configure our rollup with a centralized sequencer using the SEQUENCER_DA_ADDRESS
/// address constant. Since the centralize sequencer's address is consensus critical,
/// it has to be hardcoded as a constant, rather than read from the config at runtime.
///
/// If you want to customize the rollup to accept transactions from your own celestia
/// address, simply change the value of the SEQUENCER_DA_ADDRESS to your own address.
/// For example:
/// ```
/// const SEQUENCER_DA_ADDRESS: &str = "celestia1qp09ysygcx6npted5yc0au6k9lner05yvs9208";
/// ```
pub fn get_genesis_config<C: Context, Da: DaSpec, P: AsRef<Path>>(
    sequencer_da_address: Da::Address,
    genesis_paths: &GenesisPaths<P>,
) -> GenesisConfig<C, Da> {
    create_genesis_config(
        sequencer_da_address,
        genesis_paths
    )
    .expect("Unable to read genesis configuration")
}

fn create_genesis_config<C: Context, Da: DaSpec, P: AsRef<Path>>(
    seq_da_address: Da::Address,
    genesis_paths: &GenesisPaths<P>,
) -> anyhow::Result<GenesisConfig<C, Da>> {
  

    let mut sequencer_registry_config: SequencerConfig<C, Da> =
        read_json_file(&genesis_paths.sequencer_genesis_path)?;

    // The `seq_da_address` is overridden with the value from rollup binary.
    sequencer_registry_config.seq_da_address = seq_da_address;



    let value_setter_config: ValueSetterConfig<C> =
        read_json_file(&genesis_paths.value_setter_genesis_path)?;

    
    let reddit_config: RedditConfig = read_json_file(&genesis_paths.reddit_genesis_path)?;

    let chain_state_config: ChainStateConfig =
        read_json_file(&genesis_paths.chain_state_genesis_path)?;

   
    Ok(GenesisConfig::new(
        sequencer_registry_config,
        (),
        chain_state_config,
        value_setter_config,
        reddit_config,
    ))
}


