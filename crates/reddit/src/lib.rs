use std::ops::Sub;

use address::{PostAddress, SubAddress, UserAddress};
use post::Post;
use serde::{Deserialize, Serialize};
use sov_modules_api::{Context, Module, ModuleInfo, StateMap};
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
    post_collectoons: StateMap<PostAddress<C> , Post<C>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RedditConfig {}


impl<C:Context> Module for Reddit<C> {
     type Context = C;

    type Config = RedditConfig;

    type CallMessage = CallMessage<C>;

    type Event = ();
}