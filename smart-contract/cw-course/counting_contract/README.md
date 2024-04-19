Build

```sh
cargo build
cargo check
```

Only when you are ready to deloy the contract

```sh
cargo wasm
cosmwasm-check ./target/wasm32-unknown-unknown/release/deps/counting_contract.wasm
```

## Tutorial

- [Project-setup](https://academy.cosmwasm.com/learn/smart-contracts/prepare-a-project)