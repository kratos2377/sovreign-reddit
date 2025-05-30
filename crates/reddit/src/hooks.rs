use sov_modules_api::hooks::TxHooks;
use sov_modules_api::transaction::Transaction;
use sov_modules_api::{Context, PublicKey, Spec, WorkingSet};
use crate::Reddit;



impl<C: Context> TxHooks for Reddit<C> {
    type Context = C;
     type PreArg = C::PublicKey;
    type PreResult = C::Address;

    
    fn pre_dispatch_tx_hook(
        &self,
        tx: &Transaction<C>,
        working_set: &mut WorkingSet<C>,
           sequencer: &C::PublicKey,
    ) -> anyhow::Result<<Self::Context as Spec>::Address> {
        let pub_key = tx.pub_key();

        Ok(pub_key.to_address())
    }

    fn post_dispatch_tx_hook(
        &self,
        tx: &Transaction<Self::Context>,
        _ctx: &C,
        working_set: &mut WorkingSet<C>,
    ) -> anyhow::Result<()> {

        Ok(())
    }
    

}
