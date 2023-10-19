# rewards-center

```shell
anchor-cli : avm use 0.26.0
rustup : rustup install 1.66.1 && rustup default 1.66.1
solana-cli : sh -c "$(curl -sSfL https://release.solana.com/v1.14.16/install)"
```

```shell
rustc --version
rustc 1.66.1 (90743e729 2023-01-10)
anchor --version
anchor-cli 0.26.0
solana --version
solana-cli 1.14.16 (src:0fb2ffda; feat:3488713414)
```

## deploy
**Deploy program** : `solana program deploy --url devnet --keypair ./keypairs/update-authority.json --program-id ./keypairs/program-id.json ./target/deploy/rewards_center.so`

**Deploy idl** : `anchor idl init --filepath target/idl/trading_train_center.json --provider.cluster devnet --provider.wallet ./keypairs/update-authority.json <program-id>`

**Upgrade program** : `anchor upgrade ./target/deploy/rewards_center.so --provider.cluster devnet --program-id <program-id>`

**Upgrade idl** : `anchor idl upgrade <program-id> -f target/idl/trading_train_center.json --provider.cluster devnet`
