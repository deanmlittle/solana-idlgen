# Solana IDLGen
IDLgen generates a code scaffold for calling instructions for custom Solana program in Rust based upon its IDL.

Simply insert a compatible IDL into the `idl_gen!()` macro and you can generate:

- A program struct
- An `impl` containing callable functions to generate valid `Instructions`, as well as signed and unsigned `Transactions`
- All required data `structs` for said instructions with Borsh serialization implemented

### Quickstart

Add `solana-sdk` to your Cargo.toml then consume the macro like so:

```rs
use solana_idlgen::idlgen;
idlgen!({
    "version": "0.1.0",
    "name": "example_program",
    "instructions": [{
        "name": "example_instrution",
        "accounts": [{
            "name": "signer",
            "isMut": true,
            "isSigner": true
        }, {
            "name": "exampleAccount",
            "isMut": true,
            "isSigner": false
        }, {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
        }],
        "args": [{
            "name": "name",
            "type": "bytes"
        }]
    }],
    "accounts": [{
        "name": "ExampleAccount",
        "type": {
            "kind": "struct",
            "fields": [{
                "name": "name",
                "type": "bytes"
            }]
        }
    }],
    "metadata": {
        "address": "11111111111111111111111111111111"
    }
});
```

This will generate:
```rs
use borsh::BorshSerialize;
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[derive(Debug, BorshSerialize)]
pub struct ExampleArgs {
    pub name: Vec<u8>,
}

#[derive(Debug)]
pub struct ExampleProgram {}

impl ExampleProgram {
    pub fn id() -> Pubkey {
        Pubkey::new_from_array([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ])
    }

    pub fn example_ix_from_bytes(accounts: &[&Pubkey; 3usize], bytes: &[u8]) -> Instruction {
        let account_meta: Vec<AccountMeta> = vec![
            AccountMeta::new(accounts[0usize].clone(), true),
            AccountMeta::new(accounts[1usize].clone(), false),
            AccountMeta::new_readonly(accounts[2usize].clone(), false),
        ];
        Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
    }

    pub fn example_ix_from_data(accounts: &[&Pubkey; 3usize], args: &ExampleArgs) -> Instruction {
        let mut data_bytes: Vec<u8> = vec![189, 174, 40, 25, 180, 44, 109, 58];
        data_bytes.extend_from_slice(&args.try_to_vec().expect("Unable to serialize data"));
        Self::example_ix_from_bytes(accounts, &data_bytes)
    }

    pub fn example(
        accounts: &[&Pubkey; 3usize],
        args: &ExampleArgs,
        payer: Option<&Pubkey>,
        signers: &[&Keypair; 1usize],
        blockhash: Hash,
    ) -> Transaction {
        let ix = Self::example_ix_from_data(accounts, args);
        Transaction::new_signed_with_payer(&[ix], payer, signers, blockhash)
    }

    pub fn example_unsigned(
        accounts: &[&Pubkey; 3usize],
        args: &ExampleArgs,
        payer: Option<&Pubkey>,
    ) -> Transaction {
        let ix = Self::example_ix_from_data(accounts, args);
        Transaction::new_unsigned(Message::new(&[ix], payer))
    }
    pub fn derive_program_address(seeds: &[&[u8]]) -> Pubkey {
        Self::derive_program_address_and_bump(seeds).0
    }
    pub fn derive_program_address_and_bump(seeds: &[&[u8]]) -> (Pubkey, u8) {
        Pubkey::find_program_address(seeds, &Self::id())
    }
}

```

You can then generate PDAs and call instructions from your program like so:

```rs

// Connect to devnet RPC endpoint
let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

// Get the recent blockhash
let blockhash = rpc_client.get_latest_blockhash().unwrap();

let signer = Keypair::new();

// Derive the example PDA from ExampleProgram
let example = ExampleProgram::derive_program_address(&[&b"example".as_ref(), &signer.pubkey().as_ref()]);

// Example instruction args
let args = ExampleArgs {
    name: b"anatoly".to_vec()
};

// Call the "example" instruction of ExamplProgram
ExampleProgram::example(
    &[
        &signer.pubkey(),
        &example,
        &system_program::id()
    ],
    &args,
    Some(&signer.pubkey()),
    &[
        &signer
    ],
    blockhash
);
```

### Caveats
- you must add the `solana-sdk` and `borsh` to the Cargo.toml of the package where you consume `idlgen`
- you must populate the optional IDL `metadata -> address` field to get the Program ID