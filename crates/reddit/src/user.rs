use anyhow::{anyhow, bail};
use sov_modules_api::{Context, StateMap, StateMapAccessor, WorkingSet};

use crate::{address::{PostAddress, UserAddress}, utils::get_user_address};

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
/// Defines an nft collection
pub struct User<C: Context> {
    username: String,
    karma: u64,
    user_address: UserAddress<C>
}



impl<C: Context> User<C> {
 pub fn new(
    username: &str,
    user_collections: &StateMap<UserAddress<C> , User<C>>,
    context: &C,
    working_set: &mut WorkingSet<C>
 ) -> anyhow::Result<(UserAddress<C> , User<C>)> {


    let creator = context.sender();

    let user_address = get_user_address(username, creator.as_ref());


    let user_add = user_collections.get(&user_address, working_set);

    if user_add.is_some() {
        Err(anyhow!( "User with username={} already exists", username ))
    } else {
        Ok( (user_address , User { username: username.to_string(), karma: 0, user_address:  UserAddress::new(creator)}))
    }


 }

 #[allow(dead_code)]
 pub fn get_username(&self) -> &str {
    &self.username
 }

#[allow(dead_code)]
 pub fn get_karma(&self) -> u64 {
    self.karma
 }


 #[allow(dead_code)]
 pub fn get_user_address(&self) -> UserAddress<C> {
    self.user_address.clone()
 }

}