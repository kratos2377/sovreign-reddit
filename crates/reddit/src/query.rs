use crate::{address::{PostAddress, SubAddress, UserAddress}, utils::{get_post_address, get_sub_address, get_user_address}, Reddit};
use sov_modules_api::{Context, WorkingSet};
use sov_modules_macros::rpc_gen;
use jsonrpsee::core::RpcResult;


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "UserAddress<C>: serde::Serialize",
    deserialize = "UserAddress<C>: serde::Deserialize<'de>"
))]
/// Response for `getCollection` method
pub struct UserCollectionResponse<C: Context> {
    pub username: String,
    pub user_address: UserAddress<C>,
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "UserAddress<C>: serde::Serialize",
    deserialize = "UserAddress<C>: serde::Deserialize<'de>"
))]
/// Response for `getCollectionAddress` method
pub struct UserAddressResponse<C: Context> {
    /// Address of the collection
    pub user_address: UserAddress<C>,
}



#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "SubAddress<C>: serde::Serialize",
    deserialize = "SubAddress<C>: serde::Deserialize<'de>"
))]
pub struct SubRedditCollectionResponse<C: Context> {
    pub subname: String,
    pub desription: String,
    pub subaddress: SubAddress<C>,
    pub mods: Vec<UserAddress<C>>
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "SubAddress<C>: serde::Serialize",
    deserialize = "SubAddress<C>: serde::Deserialize<'de>"
))]
/// Response for `getCollectionAddress` method
pub struct SubAddressResponse<C: Context> {
    /// Address of the collection
    pub sub_address: SubAddress<C>,
}




#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "PostAddress<C>: serde::Serialize",
    deserialize = "PostAddress<C>: serde::Deserialize<'de>"
))]
pub struct PostCollectionResponse<C: Context> {
    pub user_address: UserAddress<C>,
    pub sub_address: SubAddress<C>,
    pub post_address: PostAddress<C>,
    pub post_title: String,
    pub content: String,
    pub flair: String,
    pub status: String
   
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "PostAddress<C>: serde::Serialize",
    deserialize = "PostAddress<C>: serde::Deserialize<'de>"
))]
pub struct PostAddressResponse<C: Context> {
    /// Address of the collection
    pub sub_address: PostAddress<C>,
}



#[rpc_gen(client, server, namespace = "reddit")]
impl<C: Context> Reddit<C> {
    #[rpc_method(name = "getUser")]
    /// Get the collection details
    pub fn get_user(
        &self,
        user_address: UserAddress<C>,
        working_set: &mut WorkingSet<C>,
    ) -> RpcResult<UserCollectionResponse<C>> {
        let c = self
            .user_collections
            .get(&user_address, working_set)
            .unwrap();

        Ok(UserCollectionResponse {
            username: c.get_username().to_string(),
            user_address: user_address.clone(),
        })
    }
    #[rpc_method(name = "getUserAddress")]
    /// Get the collection address
    pub fn get_collection_address(
        &self,
        user_add: UserAddress<C>,
        username: &str,
        _working_set: &mut WorkingSet<C>,
    ) -> RpcResult<UserAddressResponse<C>> {
        let ca = get_user_address::<C>(username, user_add.as_ref());
        Ok(UserAddressResponse {
            user_address: ca,
        })
    }





    #[rpc_method(name = "getSubreddit")]
    pub fn get_sub_reddit(
        &self,
        sub_address: SubAddress<C>,
        working_set: &mut WorkingSet<C>,
    ) -> RpcResult<SubRedditCollectionResponse<C>> {
        let c = self
            .sub_collections
            .get(&sub_address, working_set)
            .unwrap();

        Ok(SubRedditCollectionResponse { 
            subname: c.get_sub_name().to_string(), 
            desription: c.get_sub_description().to_string(), 
            subaddress: c.get_sub_address().clone(), 
            mods: c.get_mods().clone() 
        })
    }
    #[rpc_method(name = "getSubAddress")]
    pub fn get_sub_address(
        &self,
        suname: &str,
        _working_set: &mut WorkingSet<C>,
    ) -> RpcResult<SubAddressResponse<C>> {
        let ca = get_sub_address::<C>(suname);
        Ok(SubAddressResponse {
            sub_address: ca,
        })
    }




        #[rpc_method(name = "getPost")]
    pub fn get_post(
        &self,
        post_address: PostAddress<C>,
        working_set: &mut WorkingSet<C>,
    ) -> RpcResult<PostCollectionResponse<C>> {
        let c = self
            .post_collections
            .get(&post_address, working_set)
            .unwrap();

        Ok(PostCollectionResponse { 
            user_address: c.get_user_address().clone(), 
            sub_address: c.get_sub_address().clone(), 
            post_address: c.get_post_address().clone(), 
            post_title: c.get_post_title().to_string(), 
            content: c.get_post_content().to_string(), 
            flair: c.get_post_flair().to_string(), 
            status: c.get_post_status().to_string() 
        })
    }

}
