use sov_modules_api::{Context, Module, ModuleInfo};

pub mod call;
pub mod genesis;
pub mod query;


#[cfg_attr(feature = "native", derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct User<C: Context> {
    #[address]
    user_id: C::Address,

    #[state]
    username: sov_modules_api::StateValue<String>,

    #[state]
    user_joined_subs: sov_modules_api::StateMap<C::Address , String>
}

#[cfg_attr(derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct SubReddit<C: Context> {

    #[address]
    sub_id: C::Address,

    #[state]
    subname: sov_modules_api::StateValue<String>
}




#[cfg_attr(derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct Post<C: Context> {

    #[address]
    post_id: C::Address,
    #[address]
    user_id: C::Address,
    #[address]
    sub_id: C::Address,

    #[state]
    content: sov_modules_api::StateValue<String>,

    #[state]
    status: sov_modules_api::StateValue<bool>

}



#[cfg_attr(derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct Comment<C: Context> {

    #[address]
    comment_id: C::Address,
    #[address]
    user_id: C::Address,
    #[address]
    sub_id: C::Address,

    #[state]
    content: sov_modules_api::StateValue<String>,

    #[state]
    status: sov_modules_api::StateValue<bool>

}


impl<C:Context> Module for User<C> {
    
}