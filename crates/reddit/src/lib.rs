use sov_modules_api::{Context, ModuleInfo};



#[cfg_attr(derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct User<C: Context> {
    #[address]
    user_id: C::Address,

    #[state]
    username: sov_modules_api::StateValue<String>
}

#[cfg_attr(derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct SubReddit<C: Context> {

    #[address]
    sub_id: C::Address,
    #[address]
    user_id: C::Address,

    #[state]
    subname: sov_modules_api::StateValue<String>
}



#[cfg_attr(derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo, Clone)]
pub struct UserSubRelation<C: Context> {

    #[address]
    realtion_id: C::Address,
    #[address]
    user_id: C::Address,
    #[address]
    sub_id: C::Address,
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
    content: sov_modules_api::StateValue<String>

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
    content: sov_modules_api::StateValue<String>

    #[state]
    status: sov_modules_api::StateValue<bool>

}
