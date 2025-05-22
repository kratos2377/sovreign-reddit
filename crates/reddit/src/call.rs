use anyhow::{bail, Result};
#[cfg(feature = "native")]
use sov_modules_api::macros::CliWalletArg;
use sov_modules_api::{CallResponse, Context, WorkingSet};
use crate::User;



#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize),
    derive(CliWalletArg),
    derive(schemars::JsonSchema),
    schemars(bound = "C::Address: ::schemars::JsonSchema", rename = "CallMessage")
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub enum CallMessage<C: Context> {

    JoinSub {
        sub_id: C::Address,
        subname: String
    },

    LeaveSub {
        sub_id: C::Address,
    },

    CreatePost {
        user_id: C::Address,
        sub_id: C::Address,
        title: String,
        content: String,
    },

    CreateComment {
        user_id: C::Address,
        post_id: C::Address,
        content: String
    },
}

impl<C: Context> User<C> {

    pub(crate) fn join_sub_reddit(
        &self,
        sub_id: C::Address,
        subname: String,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse> {

        // if self

         if self.user_joined_subs.get(&sub_id, working_set).is_some() {
            bail!("Already joined sub={:?} for user_id={:?}", sub_id , self.user_id);
         }

         self.user_joined_subs.set(&sub_id, &subname, working_set);

         working_set.add_event("SUB-JOINED", &format!("user_id={:?} joined sub_id={:?} with sub_name={:?}" , self.user_id , sub_id , subname));
         Ok(sov_modules_api::CallResponse::default())

    }

    pub(crate) fn leave_sub_reddit(
        &self,
        sub_id: C::Address,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse> {

        // if self

         if !self.user_joined_subs.get(&sub_id, working_set).is_some() {
            bail!("No sub joined with sub_id={:?} for user_id={:?}", sub_id , self.user_id);
         }

         self.user_joined_subs.remove(&sub_id, working_set);

         working_set.add_event("SUB-LEAVED", &format!("user_id={:?} left sub_id={:?}" , self.user_id , sub_id));
         Ok(sov_modules_api::CallResponse::default())

    }

}