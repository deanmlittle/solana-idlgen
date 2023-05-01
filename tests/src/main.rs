#[cfg(test)]
mod tests {
    use solana_idlgen::{self, idl_gen};

    #[test]
    fn no_metadata() {
        idl_gen!({
            version: "0.1.0",
            name: "Test",
            instructions: [],
            accounts: []
        })            
    }
}