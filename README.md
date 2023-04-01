# Ethereum client example

Simple ethereum client in Rust to query accounts balance and get block number using [web3.rs](https://docs.rs/web3/0.18.0/web3/) and [ethers.rs](https://github.com/gakonst/ethers-rs).

### Install

- Install rust [here](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).

- You will need to add an .env file and define vars:

```
ACCOUNT_ADDRESS
HTTP_ETHEREUM
WS_ETHEREUM
```

- Build with cargo

```
cargo build
```

- Run the scripts

```
cargo run src/main.rs
```
