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
use cosmwasm_std::{StdResult, Deps, Api, Addr};

//AdminList from Item storage
use crate::state::ADMIN_LIST;


/// can_execute check this sender is admin
pub fn can_execute(deps: Deps, sender: &str) -> StdResult<bool> {
    let cfg = ADMIN_LIST.load(deps.storage)?;
    // check sender is admin or not
    let can = cfg.is_admin(&sender);
    Ok(can)
}

//map_validate validate admins addr using api trait that contain validate fn
pub fn map_validate(api: &dyn Api, admins: &[String]) -> StdResult<Vec<Addr>> {
    admins.iter().map(|addr| api.addr_validate(addr)).collect()
}