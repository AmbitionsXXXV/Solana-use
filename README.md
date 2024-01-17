# A repository about learn Solana

## [Link](https://github.com/AmbitionsXXXV/Sol-learn/blob/main/solana-program)

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
