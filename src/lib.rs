use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::{Ident, punctuated::Punctuated, token::Comma};
use convert_case::{Case, Casing};
use solana_sdk::{hash::{hash}, pubkey::Pubkey};
mod idl_types;
use idl_types::*;

fn arg_type_to_quote(arg_type: &str) -> proc_macro2::TokenStream {
    match arg_type {
        "bytes" => quote! { Vec<u8> },
        "string" => quote! { String },
        "publicKey" => quote! { Pubkey },
        "bool" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "f32" | "u64" | "i64" | "f64" | "u128" | "i128" | "u256" | "i256" => {
            let arg_type_ident = Ident::new(arg_type, proc_macro2::Span::call_site());
            quote! { #arg_type_ident }
        },
        x => panic!("Type \"{}\" not implemented yet", x)
    }
}

#[proc_macro]
#[doc = "Creates a program struct with a data struct and statically callable functions for each of its instructions."]
pub fn idlgen(input: TokenStream) -> TokenStream {
    let idl: IdlJson = serde_json::from_str(&input.to_string()).unwrap();
    let address = idl.metadata
    .as_ref()
    .and_then(|m| m.address.as_ref())
    .unwrap_or_else(|| panic!("Please make sure to define the program address in the metadata: address field."));
    
    let program_id_bytes: proc_macro2::TokenStream = Pubkey::from_str(address).expect("Invalid program address").to_bytes().iter().map(|b| TokenTree::Literal(proc_macro2::Literal::u8_unsuffixed(*b))).collect::<Punctuated<TokenTree, Comma>>().into_token_stream();
    
    let program_name = Ident::new(&format!("{}Program", idl.name.to_case(Case::Pascal)), proc_macro2::Span::call_site());

    let mut ix_structs: Vec<proc_macro2::TokenStream> = vec![];
    let mut ix_impls: Vec<proc_macro2::TokenStream> = vec![];
    
    idl.instructions.iter().for_each(|ix| {
        // Annotations for signers
        let mut signers_count: usize = 0;
        let mut signers = vec!["✍️ Required signers:\n".to_string()];
        // Annotations for accounts array
        let mut accounts_count: usize = 0;
        let mut accounts = vec!["🔑 Required accounts: \n".to_string()];
        // Inputs for function argusments
        let mut args: Vec<proc_macro2::TokenStream> = vec![];
        // Use this to construct account_meta array from account array index
        let mut account_meta: Vec<proc_macro2::TokenStream> = vec![];

        let ix_name = Ident::new(&ix.name.to_case(Case::Pascal), proc_macro2::Span::call_site());
        let ix_name_snake = Ident::new(&ix.name.to_case(Case::Snake), proc_macro2::Span::call_site());
        let ix_name_string = ix.name.clone();
        let ix_discriminator = match &ix.discriminator {
            Some(d) => d.clone(),
            None => hash(format!("global:{}", ix.name).as_bytes()).to_bytes()[0..8].to_vec()
        }.iter().map(|b| TokenTree::Literal(proc_macro2::Literal::u8_unsuffixed(*b))).collect::<Punctuated<TokenTree, Comma>>().into_token_stream();

        // Instruction function names
        let invoke = ix_name_snake.clone();
        let invoke_unsigned = Ident::new(&format!("{}_unsigned", ix_name_snake), proc_macro2::Span::call_site());
        let instruction_ix = Ident::new(&format!("{}_ix", ix_name_snake), proc_macro2::Span::call_site());
        let instruction_from_bytes = Ident::new(&format!("{}_ix_from_bytes", ix_name_snake), proc_macro2::Span::call_site());
        
        ix.accounts.iter().enumerate().for_each(|(i, account)| {
            // Setup data instantiation within the impl
            accounts_count += 1;
            match account.is_signer {
                true => {
                    signers_count += 1;
                    account_meta.push(quote! { AccountMeta::new(accounts[#i].clone(), true) });
                    accounts.push(format!("{}. {} - signer: ✅, mutable: ✅", &accounts_count, &account.name));
                    signers.push(format!("{}. {} - signer: ✅, mutable: ✅", &signers_count, &account.name));
                },
                false => match account.is_mut {
                    true => {
                        account_meta.push(quote! { AccountMeta::new(accounts[#i].clone(), false) });
                        accounts.push(format!("{}. {} - signer: ❌, mutable: ✅", &accounts_count, &account.name));
                    },
                    false => {
                        account_meta.push(quote! { AccountMeta::new_readonly(accounts[#i].clone(), false) });
                        accounts.push(format!("{}. {} - signer: ❌, mutable: ❌", &accounts_count, &account.name));
                    }
                }
            }
        });

        let accounts_string = accounts.join("\n");
        let signers_string: String = signers.join("\n");

        let arguments: Vec<IdlJsonArgument> = match &ix.args {
            Some(a) => a.clone(),
            None => vec![]
        };

        if arguments.len() > 0 {
            // Name of the data struct for our instruction
            let args_struct = Ident::new(&format!("{}Args", ix_name), proc_macro2::Span::call_site());

            arguments.iter().for_each(|arg| {
                let arg_name = Ident::new(&arg.name.to_case(Case::Snake), proc_macro2::Span::call_site());
                let arg_type = arg_type_to_quote(&arg.arg_type);
                args.push(quote! {
                    #arg_name: #arg_type
                });
            });

            ix_structs.push(quote! {
                #[doc = "The args struct for our instruction: "]
                #[doc = #ix_name_string]
                #[derive(Debug, BorshSerialize)]
                pub struct #args_struct {
                    #(pub #args),*
                }
            });

            ix_impls.push(quote! {
                #[doc = #accounts_string] 
                pub fn #instruction_from_bytes (
                    accounts: &[&Pubkey; #accounts_count], 
                    bytes: &[u8]
                ) -> Instruction {
                    let account_meta: Vec<AccountMeta> = vec![
                        #(#account_meta),*
                    ];
                    Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
                }
    
                #[doc = #accounts_string] 
                pub fn #instruction_ix (
                    accounts: &[&Pubkey; #accounts_count], 
                    args: &#args_struct
                ) -> Instruction {
                    let mut data_bytes: Vec<u8> = vec![#ix_discriminator];
                    data_bytes.extend_from_slice(&to_vec(&args).expect("Unable to serialize data"));
                    Self::#instruction_from_bytes(accounts, &data_bytes)
                }

                #[doc = #accounts_string]
                #[doc = "\n\n"]
                #[doc = #signers_string]
                pub fn #invoke (
                    accounts: &[&Pubkey; #accounts_count],
                    args: &#args_struct,
                    payer: Option<&Pubkey>,
                    signers: &[&Keypair; #signers_count],
                    blockhash: Hash
                ) -> Transaction {
                    // Create our instruction
                    let ix = Self::#instruction_ix(accounts, args);

                    // Create a TX building the transaction
                    Transaction::new_signed_with_payer(
                        &[ix],
                        payer,
                        signers,
                        blockhash
                    )
                }

                #[doc = #accounts_string]
                pub fn #invoke_unsigned (
                    accounts: &[&Pubkey; #accounts_count],
                    args: &#args_struct,
                    payer: Option<&Pubkey>,
                ) -> Transaction {
                    // Create our instruction
                    let ix = Self::#instruction_ix(accounts, args);
                    Transaction::new_unsigned(Message::new(&[ix], payer))
                }
            });
        } else {
            ix_impls.push(quote! {
                #[doc = #accounts_string] 
                pub fn #instruction_from_bytes (
                    accounts: &[&Pubkey; #accounts_count], 
                    bytes: &[u8]) -> Instruction {
                    let account_meta: Vec<AccountMeta> = vec![
                        #(#account_meta),*
                    ];
                    Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
                }
                #[doc = #accounts_string] 
                pub fn #instruction_ix (
                    accounts: &[&Pubkey; #accounts_count], 
                ) -> Instruction {
                    let data_bytes: Vec<u8> = vec![#ix_discriminator];
                    Self::#instruction_from_bytes(accounts, &data_bytes)
                }

                #[doc = #accounts_string]
                #[doc = "\n\n"]
                #[doc = #signers_string]
                pub fn #invoke (
                    accounts: &[&Pubkey; #accounts_count],
                    payer: Option<&Pubkey>,
                    signers: &[&Keypair; #signers_count],
                    blockhash: Hash
                ) -> Transaction {
                    // Create our instruction
                    let ix = Self::#instruction_ix(accounts);

                    // Create a TX building the transaction
                    Transaction::new_signed_with_payer(
                        &[ix],
                        payer,
                        signers,
                        blockhash
                    )
                }

                #[doc = #accounts_string]
                pub fn #invoke_unsigned (
                    accounts: &[&Pubkey; #accounts_count],
                    payer: Option<&Pubkey>,
                ) -> Transaction {
                    // Create our instruction
                    let ix = Self::#instruction_ix(accounts);
                    Transaction::new_unsigned(Message::new(&[ix], payer))
                }
            });
        }
    });

    let gen = quote! {
        use borsh::{BorshSerialize, to_vec};
        use solana_sdk::{signature::{Keypair, Signer}, message::Message, transaction::Transaction, hash::Hash, pubkey::Pubkey, instruction::{Instruction, AccountMeta}};

        #(#ix_structs)*

        #[derive(Debug)]
        pub struct #program_name {}

        impl #program_name {     
            #[doc = r"Returns the program ID."]
            pub fn id() -> Pubkey {
                Pubkey::new_from_array([#program_id_bytes])
            }

            #(#ix_impls)*

            pub fn derive_program_address(seeds: &[&[u8]]) -> Pubkey {
                Self::derive_program_address_and_bump(seeds).0
            }

            pub fn derive_program_address_and_bump(seeds: &[&[u8]]) -> (Pubkey, u8) {
                Pubkey::find_program_address(
                    seeds, 
                    &Self::id()
                )
            }
        }
    };
    gen.into()
}