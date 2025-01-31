use schemars::JsonSchema;    // Crate schemars: use JsonSchema to generate json for code
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;  //Crate cosmwasm_std: use Addr A human readable address is string
use cw_storage_plus::Item; //crate cw_storage_plus: Use Item for storage key and it's value 

/// Adminlist contains list of Addr of admins and flag this admin mutable or not
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AdminList {
    pub admins: Vec<Addr>,
    pub mutable: bool,
}

impl AdminList {
    /// returns true if the address is a registered admin
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        // check addr is exist in admin list
        self.admins.iter().any(|a| a.as_ref() == addr)
    }

    /// returns true if the address is a registered admin and the config is mutable
    pub fn can_modify(&self, addr: &str) -> bool {
        self.mutable && self.is_admin(addr)   // check if admin and also mutuable
    }
}
/// Admin list create new item with storage_key admin_list
pub const ADMIN_LIST: Item<AdminList> = Item::new("admin_list");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_admin() {
        let admins: Vec<_> = vec!["bob", "paul", "john"]
            .into_iter()
            .map(Addr::unchecked)
            .collect();
        let config = AdminList {
            admins: admins.clone(),
            mutable: false,
        };

        assert!(config.is_admin(admins[0].as_ref()));
        assert!(config.is_admin(admins[2].as_ref()));
        assert!(!config.is_admin("other"));
    }

    #[test]
    fn can_modify() {
        let alice = Addr::unchecked("alice");
        let bob = Addr::unchecked("bob");

        // admin can modify mutable contract
        let config = AdminList {
            admins: vec![bob.clone()],
            mutable: true,
        };
        assert!(!config.can_modify(alice.as_ref()));
        assert!(config.can_modify(bob.as_ref()));

        // no one can modify an immutable contract
        let config = AdminList {
            admins: vec![alice.clone()],
            mutable: false,
        };
        assert!(!config.can_modify(alice.as_ref()));
        assert!(!config.can_modify(bob.as_ref()));
    }
}
