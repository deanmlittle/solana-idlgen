# idlgen

Simply insert a compatible IDL into the `idl_gen!()` macro and you can generate:

- A program struct
- An `impl` containing callable functions to generate valid `Instructions`, as well as signed and unsigned `Transactions`
- All required data `structs` for said instructions with Borsh serialization implemented

### Caveats
- you must populate the optional IDL `metadata -> address` field for it to work