use anyhow::{bail, Result};
#[cfg(feature = "native")]
use sov_modules_api::macros::CliWalletArg;
use sov_modules_api::{CallResponse, Context, WorkingSet};
use crate::{user::User, Reddit, User};



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

    CreateUser {
        username: String,
    },

    CreateSubReddit {
        user_id: C::Address,
        post_id: C::Address,
        content: String
    },

    CreatePost {

    }
}

impl<C: Context> Reddit<C> {


    pub(crate) fn create_new_user(
        &self,
        username: &str,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse>{
        let (new_user_address , new_user) = User::new(username, &self.user_collections, context, working_set)?;

        self.user_collections.set(&new_user_address, &new_user, working_set);

          Ok(CallResponse::default())
    }


}