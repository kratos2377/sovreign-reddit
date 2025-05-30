use anyhow::{bail, Result};
#[cfg(feature = "native")]
use sov_modules_api::macros::CliWalletArg;
use sov_modules_api::{CallResponse, Context, StateMapAccessor, WorkingSet};
use crate::{address::{SubAddress, UserAddress}, post::Post, subreddit::SubReddit, user::User, Reddit};



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
        user_address: C::Address,
        subname: String,
        description: String,
    },

    CreatePost {
        title: String,
        flair: String,
        content: String,
        subaddress: C::Address
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

        
            //self.user_address_collections.set(context.sender(), &new_user_address, working_set);
        self.user_collections.set(&new_user_address, &new_user, working_set);

          Ok(CallResponse::default())
    }


    pub(crate) fn create_new_subreddit(
        &self,
        subname: &str,
        description: &str,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse> {


           let (new_sub_address , new_sub) = SubReddit::new(subname, description, &self.sub_collections, context, working_set)?;

        self.sub_collections.set(&new_sub_address, &new_sub, working_set);

          Ok(CallResponse::default())


    }


      pub(crate) fn create_new_post(
        &self,
        title: &str,
        flair: &str,
        content: &str,
        subaddress: SubAddress<C>,
        context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse> {


           let (new_post_address , new_post) = Post::new(title, flair, content, subaddress , context, working_set)?;

        self.post_collections.set(&new_post_address, &new_post, working_set);

          Ok(CallResponse::default())


    }






}