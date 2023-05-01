# Solana IDLGen
Idlgen generates a code scaffold for calling instructions for custom Solana program in Rust based upon its IDL.

### Getting Started
To use this template, simply clone the repository and rename it to your program's name. Then, modify the idl.json file to match the interface of your program.

```bash
$ git clone https://github.com/<your-username>/<your-program-name>.git
$ cd <your-program-name>
$ cargo build-bpf
```

### Modifying the IDL
The IDL file (idl.json) specifies the interface of your program, including its accounts, instructions, and associated data structures. Modify the IDL file to match the interface of your program.

You can then generate the program's Rust code by running:

```bash
$ cargo run --bin idl_gen < idl.json > src/program.rs
```

This will generate a src/program.rs file containing the Rust code for your program.

### Building and Deploying
After modifying the IDL file and generating the Rust code for your program, you can build and deploy it to Solana:

```bash
$ cargo build-bpf
$ solana program deploy <path-to-your-program>.so
```

### Contributing
Contributions are welcome! Please open an issue or pull request for any bugs or feature requests.

### License
This template is licensed under the MIT License. See LICENSE for more information.

# solana_idlgen

Simply insert a compatible IDL into the `idl_gen!()` macro and you can generate:

- A program struct
- An `impl` containing callable functions to generate valid `Instructions`, as well as signed and unsigned `Transactions`
- All required data `structs` for said instructions with Borsh serialization implemented

### Example


### Caveats
- you must populate the optional IDL `metadata -> address` field for it to work