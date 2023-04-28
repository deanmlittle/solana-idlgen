use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlJson {
    pub version: String,
    pub name: String,
    pub instructions: Vec<IdlJsonInstruction>,
    pub accounts: Vec<IdlJsonNewAccount>,
    pub metadata: IdlMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlMetadata {
    pub address: String
}

#[derive(Debug, Serialize , Deserialize)]
pub struct IdlJsonInstruction {
    pub name: String,
    pub accounts: Vec<IdlJsonAccount>,
    pub args: Vec<IdlJsonArgument>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlJsonAccount {
    pub name: String,
    #[serde(rename = "isMut")]
    pub is_mut: bool,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlJsonArgument {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlJsonNewAccount {
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: IdlJsonNewAccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlJsonNewAccountType {
    pub kind: String,
    pub fields: Vec<IdlJsonField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlJsonField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}