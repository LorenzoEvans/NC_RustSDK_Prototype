use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct DefaultSchemaObject{
  name: String,
  type: String,
};

impl DefaultSchemaObject {
  pub fn new() -> {
    DefaultSchemaObject {
      name: String::from("name"),
      type: String::from("name"),
    }
  }
};

#[derive(Debug)]
struct DefaultSchema {
  schema: vec![6; DefaultSchemaObject],
};

impl DefaultSchema {

  pub fn new() -> DefaultSchema {
    
    DefaultSchema {
      schema: vec![6; DefaultSchemaObject::new()]
    }
  }

  let schema_list = [
    "name", 
    "description", 
    "image", 
    "external_url", 
    "content_type",
    "content",
    "license",
  ];
  let mut DefaultSchemaObject = DefaultSchema {
    schema: vec![6; DefaultSchemaObject::new()]
  }

  { 
    for i in schema_list {
      &mut DefaultSchemaObject.schema[i].name = schema_list[i]
    }

    DefaultSchemaObject
  }

  let DefaultSchema = DefaultSchemaObject;

  DefaultSchema

  }
}

#[derive(Debug)]
struct SbtNftSchemaObject {
  name: String,
  type: String,
}
#[derive(Debug)]
struct SbtNftSchema {
  schema: vec![9; SbtNftSchemaObject]
} 

impl SbtNftSchema {

  pub fn new() -> SbtNftSchema {
    
    SbtNftSchema {
      schema: vec![9; SbtNftSchemaObject::new()]
    }
  }

  let schema_list = [
    "name", 
    "description", 
    "image", 
    "type", 
    "issuer",
    "recipient",
    "quantifiers",
    "signature",
    "content",
    "version"
  ];

  let mut SbtNftSchemaObject = SbtNftSchema {
    schema: vec![9; SbtNftSchemaObject::new()]
  }

  { 
    for i in schema_list {
      &mut SbtNftSchemaObject.schema[i].name = schema_list[i]
    }

    SbtNftSchemaObject
  }

  let SbtNftSchema = DefaultSchemaObject;

  SbtNftSchema
}

#[derive(Debug)]
struct ERC721SchemaObject {
  name: String,
  type: String,
}

#[derive(Debug)]
struct ERC721Schema {
  schema: vec![3; ERC721SchemaObject]
}

impl ERC721Schema {

  pub fn new () -> ERC721Schema {

  }
}

struct NCKeyPair  {
  pub_key: String,
  prv_key: String,
};

struct NCNameType {
  name: String,
  type: String,
};

struct NCBuyRam {
  user: String,
  payer: String,
  payer_prv_key: String,
  ram_amt: number,
};

struct NCCreateUser {
  new_user: String,
  newacc_pub_active_key: String,
  newacc_pub_owner_key: String,
  payer: String,
  payer_prv_key: String,
  ram_amt: number,
  cpu_amount: String,
  net_amount: String,
  xfer: bool, // stake or transfer CPU/NET to the account
};

struct NCCreateCollection  {
  user: String,
  user_prv_active_key: String,
  collection_name: String,
  schema_name: String,
  schema_fields: vec![],
  template_name: String,
  template_fields: vec![],
  mkt_fee: number,
  allow_notify: bool,
  xferable: bool,
  burnable: bool,
  max_supply: number,
};

struct NCCreatePermission  {
  author: String,
  perm_name: String,
  perm_pub_key: String,
  author_prv_active_key: String
};

struct NCLinkPerm  {
  author: String,               // the owner of the permission
  perm_to_link: String,
  action_owner: String,
  action_to_link: String,
  author_prv_active_key: String,
};

struct NCCreatePool  {
  owner: String,
  owner_prv_active_key: String,
  ticker: String,
  is_inflatable: bool,
  is_deflatable: bool,
  is_treasury: bool,
};

struct NCStakeMainDao  {
  amt: String,
  payer: String,
  payer_prv_key: String,
};

struct NCStakePool  {
  owner: String,
  amt: String,
  payer: String,
  payer_prv_key: String,
};

struct NCUnstakePool  {
  amt: String,
  payer: String,
  payer_prv_key: String,
};


struct NCTxNcoBal  {
  to: String,
  amt: String,
  payer: String,
  memo: String,
  payer_prv_key: String,
};

struct NCTxBal  {
  to: String,
  amt: String,
  payer: String,
  memo: String,
  payer_prv_key: String,
};

struct NCPoolInfoTotal {
  quantity: String,
  contract: String,
}
struct NCPoolInfo  {
  id: String,
  code: String,
  owner: String,
  description: String,
  total: NCPoolInfoTotal,
  creation_date: String,
  last_update_date: String,

};

struct NCPoolsInfo  {
  rows: vec![],
  more: bool,
  next_key: String,
}


struct NCCreateDao  {
  author: String,
  author_prv_key: String,
  token: String,
  descr: String,
}

struct NCCreateDaoProposal  {
  proposer: String,
  proposer_prv_key: String,
  dao_id: String,
  dao_owner: String,
  title: String,
  summary: String,
  url: String,
  vote_start: String,
  vote_end: String
};

struct NCCreateDaoUserWhitelistProposal  {
  proposer: String,
  proposer_prv_key: String,
  dao_id: String,
  dao_owner: String,
  user: String,
  vote_start: String,
  vote_end: String,
};

struct NCCreateDaoStakeProposal  {
  proposer: String,
  proposer_prv_key: String,
  dao_id: String,
  dao_owner: String,
  to: String,
  quantity: String,
  vote_start: String,
  vote_end: String,
};

struct NCApproveDaoProposal  {
  approver: String,
  approver_prv_key: String,
  dao_id: number,
  dao_owner: String,
  proposal_id: number,
  proposal_author: String,
};

struct NCExecuteDaoProposal  {
  exec: String,
  exec_prv_key: String,
  dao_id: number,
  dao_owner: String,
  proposal_id: number,
  proposal_author: String,

};

struct NCGetDaoProposals  {
  dao_id: String,
  dao_owner: String,
  proposal_id: String,
  proposal_author: String,
  lower_bound: String,
  upper_bound: String,
  limit: number,
  reverse: bool,
}

struct NCDaoProposalVote  {
  voter: String,
  voter_prv_key: String,
  dao_id: String,
  dao_owner: String,
  proposal_id: String,
  proposal_type: String,
  quantity: String,
  option: String,     // YES/NO
}

struct NCDaoWithdrawVoteDeposit  {
  voter: String,
  voter_prv_key: String,
  vote_id: String,
}

struct NCGetVotes  {
  voter: String,
  lower_bound: String,
  upper_bound: String,
  limit: String,
}

struct NCGetDaoWhiteList  {
  dao_id: String,
  dao_owner: String,
  lower_bound: String,
  upper_bound: String,
  limit: String,
  reverse: bool;
}

struct NCKeyValPair  {
  key: String,
  value: String[],
};

struct NCMintAsset  {
  creator: String,
  col_name: String,
  sch_name: String,
  tmpl_id: number,
  immutable_data: Vec<HashMap>,
  mutable_data: Vec<HashMap>,
  payer: String,
  payer_prv_key: String,
};

struct NCGetAccInfo  {
  owner: String,
  contract: String,
  token_name: String,
};                 

struct NCGetPoolInfo  {
  owner: String,
  code: String,
};

struct TxIdStakePool {
  pool_code: String,
  pool_id: String,
}
struct TxIdCreateDaoProposal {
  proposal_id: String,
  dao_id: String,
}

struct NCReturnTxs  {
  tx_id_create_acc: String,
  tx_id_create_col: String,
  tx_id_create_cch: String,
  tx_id_create_tpl: String,

  tx_id_create_perm: String,
  tx_id_link_perm: String,

  tx_id_create_pool: String,
  tx_id_stakePool: TxIdStakePool,
      // Make into struct
  tx_id_unstake_pool: String,

  tx_id_create_dao: String,
  tx_id_createDaoProposal: TxIdCreateDaoProposal,
  tx_id_approve_dao_proposal: String,
  tx_id_execute_dao_proposal: String,
  tx_id_vote_dao_proposal: String,
  tx_id_withdraw_vote_deposit: String,

  tx_id_withdraw_from_pool: String,
  tx_id_add_to_whitelist: String,
  tx_id_remove_from_whitelist: String,
  tx_id_stake_main_dao: String,
  tx_id_unstake_main_dao: String,

  tx_id_mint_asset: String,
  tx_id_tx_nco_balance: String,

  tx_id: String,
};

struct NCReturnInfo  {
  acc_balances: Vec<String>,
};

// these three schemas will have to be turned into structs,
// which will have to be impl'd, so that the default instance
// function can be written, so objects with /* name: "name" */
// can be instantiated

// export const default_schema  [
//   { name: 'name', type: "string" },
//   { name: 'description', type: "string" },
//   { name: 'image', type: 'string' },
//   { name: 'external_url', type: 'string' },
//   { name: 'content_type', type: 'string' },
//   { name: 'content', type: 'string' },
//   { name: 'license', type: 'string' }
// ];

// export const SBT_NFT_schema  [
//   { name: 'name', type: "string" },
//   { name: 'description', type: "string" },
//   { name: 'image', type: 'string' },
//   { name: 'type', type: 'string' },
//   { name: 'issuer', type: 'string' },
//   { name: 'recipient', type: 'string' },
//   { name: 'quantifiers', type: 'string' },
//   { name: 'signature', type: 'string' },
//   { name: 'content', type: 'string' },
//   { name: 'version', type: 'string' }
// ];

// https://github.com/ethereum/EIPs/blob/master/EIPS/eip-721.md
export const ERC721_schema  [
  { name: 'name', type: "string" },
  { name: 'description', type: "string" },
  { name: 'image', type: 'string' },
];

  // https://docs.opensea.io/docs/metadata-standards
  // export const OpenSea_schema  [
  //]