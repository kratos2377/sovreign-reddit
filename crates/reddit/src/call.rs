
use crate::User;

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize),
    derive(CliWalletArg),
    derive(schemars::JsonSchema),
    schemars(bound = "C::Address: ::schemars::JsonSchema", rename = "CallMessage")
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub enum CallMessage<C: Context> {
    /// Mint a new token
    CreateSubReddit {
        subname: String,
    },

    JoinSub {

    },

    UnjoinSub {

    },

    CreatePost {
        user_id: C::Address,
        sub_id: C::Address,
        title: String,
        content: String,
    },

    CreateComment {
        user_id: C::Address,
        post_id: C::Address,
        content: String
    },
}

impl<C: Context> User<C> {

    pub(crate) fn create_sub_reddit(
        &self,
        subname: String,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse> {

        if self

    }

}