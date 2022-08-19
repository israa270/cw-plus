# CW1 Whitelist

This may be the simplest implementation of CW1, a whitelist of addresses.
It contains a set of admins that are defined upon creation.
Any of those admins may `Execute` any message via the contract,
per the CW1 spec.

To make this slighly less minimalistic, you can allow the admin set
to be mutable or immutable. If it is mutable, then any admin may
(a) change the admin set and (b) freeze it (making it immutable).

While largely an example contract for CW1, this has various real-world use-cases,
such as a common account that is shared among multiple trusted devices,
or trading an entire account (used as 1 of 1 mutable). Most of the time,
this can be used as a framework to build your own,
more advanced cw1 implementations.

## Allowing Custom Messages

By default, this doesn't support `CustomMsg` in order to be fully generic
among blockchains. However, all types are Generic over `T`, and this is only
fixed in `handle`. You can import this contract and just redefine your `handle`
function, setting a different parameter to `ExecuteMsg`, and you can produce
a chain-specific message.

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1_whitelist.wasm .
ls -l cw1_whitelist.wasm
sha256sum cw1_whitelist.wasm
```

Or for a production-ready (optimized) build, run a build command in the
the repository root: https://github.com/CosmWasm/cw-plus#compiling.

### Crates Used

    ```
        In Contract.rs:

        // Crate schemars: use JsonSchema to generate json for code
        //Crate cosmwasm_std call entryPoint for main fns [instantiate -execute -query]
        //Crate cosmwasm_std  
        // to_binary convert data serialize to binary result
        // Addr Human address readable
        // Api trait Takes a human readable address and validates if it is valid
        // Binary the std_result is binary
        // CosmosMsg enum contain msg like bankMsg,staking,...
        // Deps struct contains [storage for store items - api trait that contain msg - queries]
        // Empty empty struct
        // Env struct contains info about [blockInfo - transaction info- contract]
        // MessageInfo  contains  [senders - funds is vec of coins, coin is contain deom and amount
        // Response struct contain [message is vec of subMessage - attributes is about key and value-event contains attributes -data is binary]
        // StdResult contains the result and error std

        In state.rs
        //Crate cosmwasm_std: use Addr A human readable address is string
        // Crate schemars: use JsonSchema to generate json for code
        //crate cw_storage_plus: Use Item for storage key and it's value 
    
    ```