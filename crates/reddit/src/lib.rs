use std::ops::Sub;

use address::{PostAddress, SubAddress, UserAddress};
use call::CallMessage;
use post::Post;
use serde::{Deserialize, Serialize};
use sov_modules_api::{CallResponse, Context, Error, Module, ModuleInfo, StateMap, WorkingSet};
use subreddit::SubReddit;
use user::User;

pub mod call;
pub mod genesis;
pub mod query;
pub mod user;
pub mod address;
pub mod utils;
pub mod subreddit;
pub mod post;


#[cfg_attr(feature = "native", derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct Reddit<C: Context> {
    #[address]
    address: C::Address,

    #[state]
    user_collections: StateMap<UserAddress<C>, User<C>>,

    #[state]
    sub_collections: StateMap<SubAddress<C>, SubReddit<C>>,

    #[state]
    post_collections: StateMap<PostAddress<C> , Post<C>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RedditConfig {}


impl<C:Context> Module for Reddit<C> {
     type Context = C;

    type Config = RedditConfig;

    type CallMessage = CallMessage<C>;


      fn genesis(
        &self,
        _config: &Self::Config,
        _working_set: &mut WorkingSet<C>,
    ) -> Result<(), Error> {
        Ok(())
    }


        fn call(
        &self,
        msg: Self::CallMessage,
        context: &Self::Context,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse, Error> {
        let call_result = match msg {
            CallMessage::CreateUser {
                username,
            } => self.create_new_user(&username, context, working_set),
            CallMessage::CreateSubReddit { user_address , subname , description } => {
                self.create_new_subreddit(&subname, &description, context , working_set)
            }
            CallMessage::CreatePost {
               title,
               flair,
               content,
               subaddress
            } => self.create_new_post(
                &title,
                &flair,
                &content,
                SubAddress::new(&subaddress),
                context,
                working_set,
            ),
        };
        Ok(call_result?)
    }
    
        fn charge_gas(
        &self,
        working_set: &mut WorkingSet<Self::Context>,
        gas: &<Self::Context as Context>::GasUnit,
    ) -> anyhow::Result<()> {
        working_set.charge_gas(gas)
    }

}