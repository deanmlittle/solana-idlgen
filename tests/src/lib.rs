use solana_idlgen::{self, idlgen};
// Example Program
idlgen!({
    "version": "0.1.0",
    "name": "example",
    "instructions": [{
        "name": "example",
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
        },
        {
            "name": "exampleTwo",
            "accounts": [{
                "name": "signer",
                "isMut": true,
                "isSigner": true
            }, {
                "name": "systemProgram",
                "isMut": false,
                "isSigner": false
            }],
            "args": []
        }
    ],
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

#[cfg(test)]
mod tests {
    use solana_sdk::{signature::Keypair, signer::Signer, system_program, hash::hash};
    use solana_client::rpc_client::RpcClient;

    use crate::{ExampleProgram, ExampleArgs};

    #[test]
    fn no_metadata() {
        // Connect to devnet RPC endpoint
        let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

        // Get the recent blockhash
        let blockhash = rpc_client.get_latest_blockhash().unwrap();

        let signer = Keypair::new();

        // ExampleProgram
        let example = ExampleProgram::derive_program_address(&[&b"example".as_ref(), &signer.pubkey().as_ref()]);

        // Example instruction args
        let args = ExampleArgs {
            name: b"anatoly".to_vec()
        };

        let tx_1 = ExampleProgram::example(
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

        let tx_2 = ExampleProgram::example_two(
            &[
                &signer.pubkey(),
                &system_program::id()
            ],
            Some(&signer.pubkey()),
            &[
                &signer
            ],
            blockhash
        );

        let mut example_discriminator: Vec<u8> = hash(b"global:example").to_bytes()[0..8].to_vec();
        example_discriminator.extend_from_slice(&[7,0,0,0]);
        example_discriminator.extend_from_slice(b"anatoly");
        let example_two_discriminator: Vec<u8> = hash(b"global:example_two").to_bytes()[0..8].to_vec();
        assert_eq!(example_discriminator, tx_1.message.instructions[0].data);
        assert_eq!(example_two_discriminator, tx_2.message.instructions[0].data);
    }
}
 