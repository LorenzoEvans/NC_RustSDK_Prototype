use nc_sdk_types::{NCNameType};

enum EosioData {
    Some(data),
    None,
}

struct EosioAuthorizationObject = {
    actor: String,
    permission: String,
};

impl EosioAuthorizationOboject {

    pub fn new() -> EosioAuthorizationObject {
        actor: String::from("new"),
        permission: String::from
    }
}


struct EosioActionObject = {
    account: String,
    name: String,
    authorization: Vec<EosioAuthorizationObject>,
    data: EosioData
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

struct ActionGenerator {
    
}

#[cfg(test)]
mod tests {
    #[test]
}