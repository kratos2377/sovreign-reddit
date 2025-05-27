use std::collections::HashMap;
use anyhow::{anyhow, bail};
use sov_modules_api::{Context, StateMap, WorkingSet};

use crate::{address::{SubAddress, UserAddress}, utils::get_sub_address};




#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
/// Defines an nft collection
pub struct SubReddit<C: Context> {
    subaddress: SubAddress<C>,
    subname: String,
    description: String,
    mods: Vec<UserAddress<C>>
}

impl<C: Context> SubReddit<C> {


 pub fn new(
    subname: &str,
    description: &str,
    sub_collections: &StateMap<SubAddress<C> , SubReddit<C>>,
    context: &C,
    working_set: &mut WorkingSet<C>
 ) -> anyhow::Result<(SubAddress<C> , SubReddit<C>)> {

         let creator = context.sender();

    let sub_address = get_sub_address(subname);


    let sub_add = sub_collections.get(&sub_address, working_set);

    if sub_add.is_some() {
        Err(anyhow!( "Subreddit with subname={} already exists", subname ))
    } else {
        Ok(  
            (sub_address.clone() , SubReddit {
             subaddress: sub_address.clone(), 
            subname: subname.to_string(), 
            description: description.to_string(), 
            mods: vec![UserAddress::new(creator)] })

         )
    }


    }


    #[allow(dead_code)]
    pub fn get_sub_name(&self) -> &str {
        &self.subname
    }


        #[allow(dead_code)]
    pub fn get_sub_description(&self) -> &str {
        &self.description
    }


        #[allow(dead_code)]
    pub fn get_sub_address(&self) -> &SubAddress<C> {
        &self.subaddress
    }


           #[allow(dead_code)]
    pub fn get_mods(&self) -> &Vec<UserAddress<C>> {
        &self.mods
    }


}