use cosmwasm_std::Empty;
use schemars::JsonSchema;




pub trait CustomMsg: Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl CustomMsg for Empty {}

pub trait WhiteListNative<C>: WhiteListNativeExecute<C> + WhiteListNativeQuery
where
    C: CustomMsg,
{
}

pub trait WhiteListNativeExecute<C>
where
    C: CustomMsg,
{
    type Err: ToString;


}