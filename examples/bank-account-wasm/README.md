# Bank Account WASM Example

A bank account aggregate using an ESDL file which compiles to WASM.

## Build wasm

CD into this directory

```bash
$ cd examples/bank-account-wasm
```

Install wasm32-wasi toolchain

```bash
$ rustup target add wasm32-wasi
```

Build to WASM

```bash
cargo build --target wasm32-wasi --release
```

The resulting WASM file can be found at `./target/wasm32-wasi/release/bank_account_wasm.wasm`.

## Wasmtime

The aggregate can be used in Rust with [wasmtime](https://github.com/bytecodealliance/wasmtime).

```rust
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wit_bindgen_wasmtime::import;

import!("./domain.wit");

pub use domain::{Domain, DomainData};

struct Data {
    wasi: wasmtime_wasi::WasiCtx,
    domain: DomainData,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::<Data>::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(
        &engine,
        Data {
            wasi,
            domain: DomainData::default(),
        },
    );

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "target/wasm32-wasi/release/bank_account_wasm.wasm")?;

    let (domain, _instance) =
        Domain::instantiate(&mut store, &module, &mut linker, move |ctx| &mut ctx.domain)?;

    linker.module(&mut store, "", &module)?;

    // Create a new aggregate instance
    let mut state = domain.new_instance(&mut store, "john-doe")??;

    // Open the account
    let events = domain.handle_command(
        &mut store,
        &state,
        br#"{"command":"open_account","params":{"initial_balance":10.0}}"#,
    )??;

    // Apply opened account event and get new state
    state = domain.apply_events(
        &mut store,
        &state,
        &events
            .iter()
            .map(|event| event.as_ref())
            .collect::<Vec<_>>(),
    )??;

    // Deposit 50.0
    let events = domain.handle_command(
        &mut store,
        &state,
        br#"{ "command": "deposit_funds", "params": { "amount":50.0 } }"#,
    )??;

    // Apply events
    state = domain.apply_events(
        &mut store,
        &state,
        &events
            .iter()
            .map(|event| event.as_ref())
            .collect::<Vec<_>>(),
    )??;

    // Print events to terminal
    let events = events
        .into_iter()
        .map(|event| String::from_utf8_lossy(&event).to_string())
        .collect::<Vec<_>>();
    println!("{:?}", events);

    Ok(())
}

impl std::error::Error for domain::Error {}

impl std::fmt::Display for domain::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            domain::Error::Command(bytes) => write!(f, "{}", String::from_utf8_lossy(bytes)),
            _ => write!(f, "{:?}", self),
        }
    }
}
```

---

Commands are sent in the following format:

```json
{
  "command": "command_name",
  "params": {
    ...
  }
}
```

```rust
struct Command {
    command: String,
    params: serde_json::Map<String, serde_json::Value>,
}
```

Events are sent in the following format:

```json
{
  "event": "EventName",
  "data": {
    ...
  }
}
```

```rust
struct Event {
    event: String,
    data: serde_json::Map<String, serde_json::Value>,
}
```


