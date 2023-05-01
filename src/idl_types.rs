use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlJson {
    pub version: String,
    pub name: String,
    pub instructions: Vec<IdlJsonInstruction>,
    pub accounts: Option<Vec<IdlJsonNewAccount>>,
    pub metadata: Option<IdlMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlMetadata {
    pub address: Option<String>
}

#[derive(Debug, Clone, Serialize , Deserialize)]
pub struct IdlJsonInstruction {
    pub name: String,
    pub accounts: Vec<IdlJsonAccount>,
    #[serde(default)]
    pub args: Option<Vec<IdlJsonArgument>>,
    #[serde(default)]
    pub discriminator: Option<Vec<u8>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlJsonAccount {
    pub name: String,
    #[serde(rename = "isMut")]
    pub is_mut: bool,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlJsonArgument {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlJsonNewAccount {
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: IdlJsonNewAccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlJsonNewAccountType {
    pub kind: String,
    pub fields: Vec<IdlJsonField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlJsonField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}