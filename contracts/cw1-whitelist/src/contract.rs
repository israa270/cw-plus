




#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;     //Crate cosmwasm_std call entryPoint for main fns [instantiate -execute -query]
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
 use cosmwasm_std::{  
     Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,to_binary
};

// Crate cw2 -set_contract_version -store version name and num from cargo tomal
use cw2::set_contract_version;
//Crate error.rs ContractError struct 
use crate::error::ContractError;
//Crate msg.rs these struct used in request and response
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
//Crate state.rs 
//AdminList struct is object of state.
// Admin_list is type of Item storage that store key and value
use crate::state::{AdminList, ADMIN_LIST};

use crate::whitelist::{WhiteListContract,WhiteListExecute,WhiteListQuery};

use crate::whitelist_helper::map_validate;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw1-whitelist";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// entry_point that tell rust to start with instantiate fn
/// instantiate fn is about startup code and initialize Adminlist state and store it in Item storage.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // set the contract version and store it what's contract name and version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
     //instantiate Adminlist object
    let cfg = AdminList {
        admins: map_validate(deps.api, &msg.admins)?,  // validate admin addr 
        mutable: msg.mutable,   // admin mutuable or not
    };
    //save adminlist in Item storage
    ADMIN_LIST.save(deps.storage, &cfg)?;

    Ok(Response::default())  // return the response default
}

/// entry_point that tell rust to start with execute fn is about write operation in contract
/// execute fn execut  ExecuteMsg enum has three cases 
/// execute is about authorize admin address
/// freeze made is admin as immutable
/// update admin if is mutable if not mutable it can't update it.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    // Note: implement this function with different type to add support for custom messages
    // and then import the rest of this contract code.
    msg: ExecuteMsg<Empty>,
) -> Result<Response<Empty>, ContractError> {
    // instatiate contract from WhitList Contract default  Used to call fn in WhitelistExecut trait
    let contract = WhiteListContract::<Empty, Empty, Empty>::default();
    
    //match case of Execute Msg
    match msg {
        ExecuteMsg::Execute { msgs } => contract.execute_execute(deps, env, info, msgs), //  execute is about authorize admin address
        ExecuteMsg::Freeze {} => contract.execute_freeze(deps, env, info), //freeze made is admin as immutable
        ExecuteMsg::UpdateAdmins { admins } => contract.execute_update_admins(deps, env, info, admins), // update admin if is mutable if not mutable it can't update it.
    }
}


/// entry_point that tell rust to start with query fn to read operation from contract
/// query match cases of QueryMsg contains
/// adminlist get admin list data
/// canExecute check if admin is admin or not
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
      // instatiate contract from WhitList Contract default  Used to call fn in WhitelistExecut trait
      let contract = WhiteListContract::<Empty, Empty, Empty>::default();
    //match case from QueryMsg 
    match msg {
        QueryMsg::AdminList {} => to_binary(&contract.query_admin_list(deps)?),  // get adminList data and convert it to_binary 
        QueryMsg::CanExecute { sender, msg } => to_binary(&contract.query_can_execute(deps, sender, msg)?), // check sender is admin or not
    }
}


#[cfg(test)]
mod tests {
    use crate::msg::AdminListResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, BankMsg, StakingMsg, SubMsg, WasmMsg, to_binary, CosmosMsg};

    #[test]
    fn instantiate_and_modify_config() {
        let mut deps = mock_dependencies();

        let alice = "alice";
        let bob = "bob";
        let carl = "carl";

        let anyone = "anyone";

        // instantiate the contract
        let instantiate_msg = InstantiateMsg {
            admins: vec![alice.to_string(), bob.to_string(), carl.to_string()],
            mutable: true,
        };
        let info = mock_info(anyone, &[]);
        instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

        // ensure expected config
        let expected = AdminListResponse {
            admins: vec![alice.to_string(), bob.to_string(), carl.to_string()],
            mutable: true,
        };
          // instatiate contract from WhitList Contract default  Used to call fn in WhitelistExecut trait
        let contract = WhiteListContract::<Empty, Empty, Empty>::default();
        assert_eq!(contract.query_admin_list(deps.as_ref()).unwrap(), expected);

        // anyone cannot modify the contract
        let msg = ExecuteMsg::UpdateAdmins {
            admins: vec![anyone.to_string()],
        };
        let info = mock_info(anyone, &[]);
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // but alice can kick out carl
        let msg = ExecuteMsg::UpdateAdmins {
            admins: vec![alice.to_string(), bob.to_string()],
        };
        let info = mock_info(alice, &[]);
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // ensure expected config
        let expected = AdminListResponse {
            admins: vec![alice.to_string(), bob.to_string()],
            mutable: true,
        };
        assert_eq!(contract.query_admin_list(deps.as_ref()).unwrap(), expected);

        // carl cannot freeze it
        let info = mock_info(carl, &[]);
        let err = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Freeze {}).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // but bob can
        let info = mock_info(bob, &[]);
        execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Freeze {}).unwrap();
        let expected = AdminListResponse {
            admins: vec![alice.to_string(), bob.to_string()],
            mutable: false,
        };
        assert_eq!(contract.query_admin_list(deps.as_ref()).unwrap(), expected);

        // and now alice cannot change it again
        let msg = ExecuteMsg::UpdateAdmins {
            admins: vec![alice.to_string()],
        };
        let info = mock_info(alice, &[]);
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }

    #[test]
    fn execute_messages_has_proper_permissions() {
        let mut deps = mock_dependencies();

        let alice = "alice";
        let bob = "bob";
        let carl = "carl";

        // instantiate the contract
        let instantiate_msg = InstantiateMsg {
            admins: vec![alice.to_string(), carl.to_string()],
            mutable: false,
        };
        let info = mock_info(bob, &[]);
        instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

        let freeze: ExecuteMsg<Empty> = ExecuteMsg::Freeze {};
        let msgs = vec![
            BankMsg::Send {
                to_address: bob.to_string(),
                amount: coins(10000, "DAI"),
            }
            .into(),
            WasmMsg::Execute {
                contract_addr: "some contract".into(),
                msg: to_binary(&freeze).unwrap(),
                funds: vec![],
            }
            .into(),
        ];

        // make some nice message
        let execute_msg = ExecuteMsg::Execute { msgs: msgs.clone() };

        // bob cannot execute them
        let info = mock_info(bob, &[]);
        let err = execute(deps.as_mut(), mock_env(), info, execute_msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // but carl can
        let info = mock_info(carl, &[]);
        let res = execute(deps.as_mut(), mock_env(), info, execute_msg).unwrap();
        assert_eq!(
            res.messages,
            msgs.into_iter().map(SubMsg::new).collect::<Vec<_>>()
        );
        assert_eq!(res.attributes, [("action", "execute")]);
    }

    #[test]
    fn can_execute_query_works() {
        let mut deps = mock_dependencies();

        let alice = "alice";
        let bob = "bob";

        let anyone = "anyone";

        // instantiate the contract
        let instantiate_msg = InstantiateMsg {
            admins: vec![alice.to_string(), bob.to_string()],
            mutable: false,
        };
        let info = mock_info(anyone, &[]);
        instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

        // let us make some queries... different msg types by owner and by other
        let send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: anyone.to_string(),
            amount: coins(12345, "ushell"),
        });
        let staking_msg = CosmosMsg::Staking(StakingMsg::Delegate {
            validator: anyone.to_string(),
            amount: coin(70000, "ureef"),
        });

        // instatiate contract from WhitList Contract default  Used to call fn in WhitelistExecut trait
        let contract = WhiteListContract::<Empty, Empty, Empty>::default();
        
        // owner can send
        let res = contract.query_can_execute(deps.as_ref(), alice.to_string(), send_msg.clone()).unwrap();
        assert!(res.can_execute);

        // owner can stake
        let res = contract.query_can_execute(deps.as_ref(), bob.to_string(), staking_msg.clone()).unwrap();
        assert!(res.can_execute);

        // anyone cannot send
        let res = contract.query_can_execute(deps.as_ref(), anyone.to_string(), send_msg).unwrap();
        assert!(!res.can_execute);

        // anyone cannot stake
        let res = contract.query_can_execute(deps.as_ref(), anyone.to_string(), staking_msg).unwrap();
        assert!(!res.can_execute);
    }
}
