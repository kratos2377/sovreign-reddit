use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sov_modules_api::digest::Digest;
use uuid::Uuid;

use crate::address::{PostAddress, SubAddress, UserAddress};




pub fn get_user_address<C: sov_modules_api::Context>(
    name: &str,
    sender: &[u8],
) -> UserAddress<C> {
    let mut hasher = C::Hasher::new();
    hasher.update(sender);
    hasher.update(name.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    UserAddress::new(&C::Address::from(hash))
}


pub fn get_sub_address<C: sov_modules_api::Context>(
    subname: &str
) -> SubAddress<C> {
    let mut hasher = C::Hasher::new();

    hasher.update(subname.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    SubAddress::new(&C::Address::from(hash))
}


pub fn get_post_address<C: sov_modules_api::Context>(
    user_address: &[u8],
    sub_address: &[u8],
) -> PostAddress<C> {
    let new_uuid = Uuid::new_v4();
    let mut hasher = C::Hasher::new();

    hasher.update(user_address);
    hasher.update(sub_address);
    hasher.update(new_uuid.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    PostAddress::new(&C::Address::from(hash))
}