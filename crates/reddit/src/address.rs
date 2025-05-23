use std::fmt;

use sov_modules_api::Context;
use sov_modules_macros::address_type;

#[address_type]
pub struct UserAddress;

#[address_type]
pub struct SubAddress;

#[address_type]
pub struct PostAddress;

#[address_type]
pub struct CommentAddress;