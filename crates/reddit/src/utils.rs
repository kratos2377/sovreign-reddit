use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sov_modules_api::digest::Digest;

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
    title: &str
) -> PostAddress<C> {
    let mut hasher = C::Hasher::new();

    hasher.update(user_address);
    hasher.update(sub_address);
    hasher.update(title.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    PostAddress::new(&C::Address::from(hash))
}