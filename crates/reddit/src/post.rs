use sov_modules_api::{Context, WorkingSet};

use crate::{address::{PostAddress, SubAddress, UserAddress}, user::User, utils::get_post_address};



#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
/// Defines an nft collection
pub struct Post<C: Context> {
    post_address: PostAddress<C>,
    user_address: UserAddress<C>,
    subaddress: SubAddress<C>,
    post_title: String,
    flair: String,
    content: String,
    status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostStatus{
    ACTIVE,
    ARCHIVED,
    DELETED
}

impl PostStatus {


        fn to_string(&self) -> String {
        match self {
            PostStatus::ACTIVE => "ACTIVE".to_string(),
            PostStatus::ARCHIVED => "ARCHIVED".to_string(),
            PostStatus::DELETED => "DELETED".to_string(),
        }
    }

    // Convert string to enum
    fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "ACTIVE" => Ok(PostStatus::ACTIVE),
            "ARCHIVED" => Ok(PostStatus::ARCHIVED),
            "DELETED" => Ok(PostStatus::DELETED),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }

}

impl<C: Context> Post<C> {
    pub fn new(
        title: &str,
        flair: &str,
        content: &str,
        sub_address: SubAddress<C>,
    context: &C,
    working_set: &mut WorkingSet<C>
 ) -> anyhow::Result<Post<C>> {


        let creator = context.sender();

    let post_address = get_post_address(creator.as_ref() ,
     sub_address.as_ref() , title );

    Ok(Post {
        post_address,
        user_address: UserAddress::new(creator),
        subaddress: sub_address,
        post_title: title.to_string(),
        flair: flair.to_string(),
        content: content.to_string(),
        status: PostStatus::ACTIVE.to_string(),
    })
    


 }
}