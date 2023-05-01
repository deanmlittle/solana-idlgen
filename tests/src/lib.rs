use solana_idlgen::{self, idlgen};
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

#[cfg(test)]
mod tests {
    use solana_sdk::{signature::Keypair, signer::Signer, system_program};
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
    }
}
 