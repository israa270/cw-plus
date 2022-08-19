

use std::{marker::PhantomData};  // fmt Debug for display the object
 //Crate cosmwasm_std  
 // to_binary convert data serialize to binary result
 // Addr Human address readable
 // Api trait Takes a human readable address and validates if it is valid
 // Binary the std_result is binary
 // CosmosMsg enum contain msg like bankMsg,staking,...
 // Deps struct contains [storage for store items - api trait that contain msg - queries]
 // Empty empty struct
 // Env struct contains info about [blockInfo - transaction info- contract]
 // MessageInfo  contains  [senders - funds is vec of coins, coin is contain deom and amount
 // Response struct contain [message is vec of subMessage - attributes is about key and value-event contains attributes -data is binary]
 // StdResult contains the result and error std
use cosmwasm_std::{Empty, DepsMut, Env, MessageInfo, Response, StdResult, Deps, CosmosMsg};
use cw1::CanExecuteResponse; // Crate cw1 - CanExecuteResponse check if execute or not
use schemars::JsonSchema;  // Crate schemars: use JsonSchema to generate json for code
//Crate msg.rs these struct used in request and response
use crate::{ContractError, whitelist_helper::{can_execute, map_validate}, state::ADMIN_LIST, msg::AdminListResponse};

/// CustomMsg trait implement for clone that can copy , Debug that can display 
pub trait CustomMsg: Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl CustomMsg for Empty {}

/// WhiteList trait for execute and query fns
pub trait WhiteList<C,E,Q>: WhiteListExecute<C> + WhiteListQuery
where
    C: CustomMsg,
    // E: CustomMsg,
    // Q: CustomMsg,
{
}

/// WhiteListExecute trait execute used to fns in write operation to contract
pub trait WhiteListExecute<C>
where
    C: CustomMsg,
    // E: CustomMsg,
    // Q: CustomMsg,
    // T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    type Err: ToString;
    /// execute is about authorize admin address
    fn execute_execute(&self, deps: DepsMut, env: Env ,info: MessageInfo, msgs: Vec<cosmwasm_std::CosmosMsg<C>>) -> Result<Response<C>, Self::Err>;
    /// execute_freeze freeze made is admin as immutable
    fn execute_freeze(&self, deps: DepsMut, env: Env ,info: MessageInfo) -> Result<Response<C>, Self::Err>;
     /// execute_update_admins update admin if is mutable if not mutable it can't update it.
    fn execute_update_admins(&self, deps: DepsMut, env: Env ,info: MessageInfo, admins: Vec<String>) -> Result<Response<C>, Self::Err>;
}

/// WhiteListQuery  trait for query fn match to read operation from contract
pub trait WhiteListQuery {
    /// query_admin_list return data of all admins in contract
   fn query_admin_list(&self, deps: Deps) -> StdResult<AdminListResponse>;
   /// query_can_execute check sender is admin or not
   fn query_can_execute(&self,deps: Deps,sender: String,msg: CosmosMsg) -> StdResult<CanExecuteResponse>; 
}

/// WhiteListContract struct for custom generic type
pub struct WhiteListContract<C,E,Q>
where
    C: CustomMsg,
    // E: CustomMsg,
    // Q: CustomMsg,
    // T: Clone + fmt::Debug + PartialEq + JsonSchema,
 {
    //keys address and denom
    //  WhiteLists: Map<'a, (&'a str, &'a str), State>,
    pub(crate) _custom_response: PhantomData<C>,
    pub(crate) _custom_execute: PhantomData<E>,
    pub(crate) _custom_query: PhantomData<Q>,
    // pub(crate)  _custom_state: PhantomData<T>,
}

/// implement Default fn values for  WhiteListContract
impl<C, E, Q> Default for WhiteListContract<C,E,Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
    // T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    fn default() -> Self {
        Self::new(
            // "WhiteLists",
        )
    }
}
/// implement new for WhiteListContract object 
impl<C,E,Q> WhiteListContract<C,E,Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
    // T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    fn new(
        // WhiteList_key: &'a str,
    ) -> Self {
        Self {
            // WhiteLists: Map::new(WhiteList_key),
            _custom_response: PhantomData,
            _custom_execute: PhantomData,
            _custom_query: PhantomData,
            // _custom_state: PhantomData,
        }
    }
}

///implement  WhiteListExecute execute  fns implement for  WhiteListContract 
impl<C,E,Q> WhiteListExecute<C> for WhiteListContract<C,E,Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
    // T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    type Err = ContractError;
   
    
    /// execute is about authorize admin address
     fn execute_execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msgs: Vec<CosmosMsg<C>>,
    ) ->  Result<Response<C>, Self::Err>{
        // can_execute check this sender is admin if not this addr or sender is not authorized
        if !can_execute(deps.as_ref(), info.sender.as_ref())? {
            Err(ContractError::Unauthorized {})
        } else {
            // response object contains msg is return msg from type cosmosMsg and attributes is about key, value action with value execute
            let res = Response::new()
                .add_messages(msgs)      
                .add_attribute("action", "execute");
            Ok(res)
        }
    }

    /// execute_freeze freeze made is admin as immutable
     fn execute_freeze(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response<C>, Self::Err> {
        let mut cfg = ADMIN_LIST.load(deps.storage)?;
        // returns true if the address is a registered admin and the config is mutable
        if !cfg.can_modify(info.sender.as_ref()) {
            Err(ContractError::Unauthorized {})
        } else {
            cfg.mutable = false;  //change mutable for sender
            //store updateAdmin with new value mutable in Item storage
            ADMIN_LIST.save(deps.storage, &cfg)?;
            // response the attribute contains action is freeze
            let res = Response::new().add_attribute("action", "freeze");
            // return the res 
            Ok(res)
        }
    }

    /// execute_update_admins update admin if is mutable if not mutable it can't update it.
     fn execute_update_admins(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> Result<Response<C>, Self::Err>{ 
        //load admin from storage Item
        let mut cfg = ADMIN_LIST.load(deps.storage)?;
        // check if admin can modify or not 
        if !cfg.can_modify(info.sender.as_ref()) {
            Err(ContractError::Unauthorized {})
        } else {
            //validate addr of admins using api trait
            cfg.admins = map_validate(deps.api, &admins)?;
            // save admin in cfg updated after map_validate
            ADMIN_LIST.save(deps.storage, &cfg)?;
        // response is add attributes action is update admin
            let res = Response::new().add_attribute("action", "update_admins");
        // return res
            Ok(res)
        }
    }

}

/// impplement WhiteListQuery  for whitelist contract struct for implement fn in query trait 
impl<C,E,Q> WhiteListQuery for WhiteListContract<C,E,Q>
where
    C: CustomMsg,     // c custom msg
    E: CustomMsg,    // E execute
    Q: CustomMsg,    // Q query
    // T: Clone + fmt::Debug + PartialEq + JsonSchema,  // T state
{

        /// query_admin_list return data of all admins in contract
    fn query_admin_list(&self,deps: Deps) -> StdResult<AdminListResponse> {
        // load admins from storage Item
        let cfg = ADMIN_LIST.load(deps.storage)?;
        //return AdminListResponse object
        Ok(AdminListResponse {
            admins: cfg.admins.into_iter().map(|a| a.into()).collect(),  // collect admins and return it in vec
            mutable: cfg.mutable,
        })
    }

    /// query_can_execute check sender is admin or not
    fn query_can_execute(
        &self,
        deps: Deps,
        sender: String,
        _msg: CosmosMsg,
    ) -> StdResult<CanExecuteResponse> {
        // return CanExecuteResponse is contain bool is execute or not
        Ok(CanExecuteResponse {
            can_execute: can_execute(deps, &sender)?,
        })
    }
}