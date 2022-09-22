use nc_sdk_types::{NCNameType};
use std::fmt;

#[derive(Debug)]
enum EosioData {
    Some(data),
    None,
}
#[derive(Debug)]
struct EosioAuthorizationObject = {
    actor: String,
    permission: String,
};

impl fmt::Display for EosioAuthorizationObject {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "actor: {}, permission: {}.", self.actor, self.permission)
    }
}

impl EosioAuthorizationOboject {

    pub fn new() -> EosioAuthorizationObject {
        actor: String::from("new"),
        permission: String::from
    }
}

#[derive(Debug)]
struct EosioActionObject = {
    account: String,
    name: String,
    authorization: Vec<EosioAuthorizationObject>,
    data: EosioData
}

impl fmt::Display for EosioActionObject {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
        "account: {},
        name: {},
        authorization: {},
        data: {}.
        ", 
        self.account, 
        self.name,
        self.authorization,
        self.data)
    }
}

impl EosioActionObject {
    pub fn new () -> EosioActionObject {
        EosioActionObject {
            account: String::from("new_account"),
            name: String::from("name"),
            authorization: Vec<EosioAuthorizationObject>,
            data: EosioData,
        }
    }
}

// struct ActionGenerator {

// }

#[cfg(test)]
mod tests {
    #[test]
}