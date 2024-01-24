# A repository about learn Solana

## [Simple entry solana program](https://github.com/AmbitionsXXXV/Sol-learn/tree/main/app/solana-program)

### run

```shell
# run solana on-chain program
cd solana-program/hello_world
cargo build-bpf
solana program deploy ./target/deploy/hello_world.so
```

```shell
# copy the program id from the output of the above command
cd solana-program
# if need, uncomment the `airdropSolIfNeeded` function's execution
npm run start
```
