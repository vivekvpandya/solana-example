### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```

### Design
#### invest() 
    - user A sends funds in terms of USDC.
    - user A also has address of token A.
    - program sends USDC to admin/fund's USDC account.
    - program mints tokens equal to value of USDC recieved. (TODO: for now it is assumed that tokens are pre-minted.)
    - program transfers tokens to user A's token account.
    - The fund has a primary solana account which is managed by owner of the fund.
    - The fund also has a USDC spl-token account and fund-token spl-token account.
    - For both above spl-token account fund's primary account is authority account.
