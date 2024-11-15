set dotenv-load := true

#test contracts using forge
build-contracts:
    forge install --root ./contracts
    forge test --root ./contracts

build-amm-contracts:
    forge compile --root ./crates/amms-rs/contracts

#test contracts using forge
test-contracts: 
    forge test --root ./contracts

generate-bindings:
    #!/usr/bin/env bash
    bindings_path="./crates/bindings"
    contract_root_path="./crates/strategies/uni-tri-arb/contracts/"
    rm -rf $bindings_path
    forge bind --bindings-path $bindings_path --root $contract_root_path --crate-name bindings --force --skip-cargo-toml --alloy

generate-executor-binding:
    #!/usr/bin/env bash
    bindings_path="./crates/executor-binding"
    contract_root_path="./contracts"
    rm -rf $bindings_path
    forge bind --bindings-path $bindings_path --root $contract_root_path --crate-name executor-binding --alloy --alloy-version v0.5.4 --via-ir

fmt: 
    cargo +nightly fmt --all

clippy: 
    cargo clippy --all --all-features

setup-db:
    diesel setup --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-new-migration NAME:
    diesel migration generate {{NAME}} --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-revert:
    diesel migration revert --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-migrate:
    diesel migration run --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-print:
    diesel print-schema -- --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

run-bot-arbitrum:
    RUST_BACKTRACE=1 cargo run --bin bot -- --chain-id 42161

run-bot-mainnet:
    cargo run --bin bot -- --chain-id 1

get-filtered-pools CHAIN_ID:
    cargo run --bin cli -- filter --chain-id {{CHAIN_ID}}

get-uni-v3-pools CHAIN_ID EXCHANGE_NAME STEP START_BLOCK:
    cargo run --bin cli -- get-uniswap-v3-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}} --from-block {{START_BLOCK}} --step {{STEP}}

get-uni-v2-pools CHAIN_ID EXCHANGE_NAME:
    cargo run --bin cli -- get-uniswap-v2-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}}

get-contract-creation-block CHAIN_ID CONTRACT_ADDRESS:
    cargo run --bin cli -- get-contract-creation-block --chain-id {{CHAIN_ID}} --contract-address {{CONTRACT_ADDRESS}}

# ## TODO. make this command interactive
# get-pools:
#     # mainnet
#     cargo run --bin cli -- get-uniswap-v3-pools --chain-id 1 --exchange sushiswap-v3 --from-block {{START_BLOCK}} --to-block 20709779 --step 50000
#     cargo run --bin cli -- get-uniswap-v3-pools --chain-id 1 --exchange uniswap-v3 --from-block {{START_BLOCK}} --to-block 20709779 --step 50000
#     cargo run --bin cli -- get-uniswap-v2-pools --chain-id 1 --exchange uniswap-v2
#     cargo run --bin cli -- get-uniswap-v2-pools --chain-id 1 --exchange sushiswap-v2
#     # arbitrum
#     cargo run --bin cli -- get-uniswap-v3-pools --chain-id 42161 --exchange uniswap-v3 --from-block {{START_BLOCK}} --to-block 20709779 --step 50000
#     cargo run --bin cli -- get-uniswap-v3-pools --chain-id 42161 --exchange sushiswap-v3 --from-block {{START_BLOCK}} --to-block 20709779 --step 50000
#     cargo run --bin cli -- get-uniswap-v2-pools --chain-id 42161 --exchange uniswap-v2
#     cargo run --bin cli -- get-uniswap-v2-pools --chain-id 42161 --exchange sushiswap-v2

get-amm-value CHAIN_ID POOL_ADDRESS:
    cargo run --bin cli -- get-amm-value --chain-id {{CHAIN_ID}} --pool-address {{POOL_ADDRESS}}

activate-pools CHAIN_ID EXCHANGE_NAME:
    cargo run --bin cli -- activate-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}} --min-usd 20000

start-anvil-base:
    anvil --chain-id 8453 --fork-url https://base-mainnet.g.alchemy.com/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --steps-tracing

start-anvil-arbitrum:
    anvil --chain-id 42161 --fork-url https://arb-mainnet.g.alchemy.com/v2/-FDfJ1GYdKyvmVXVfQLTbr_7i04YGMKU --steps-tracing

start-anvil-optimism:
    anvil --chain-id 10 --fork-url https://opt-mainnet.g.alchemy.com/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --steps-tracing

start-anvil-ethereum:
    anvil --chain-id 1 --fork-url https://eth-mainnet.alchemyapi.io/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --steps-tracing

deploy-executor-local:
    cd contracts && forge script ./script/DeployBatchExecutor.s.sol:DeployBatchExecutor --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast -vv

deploy-executor-arbitrum:
    cd contracts && forge script ./script/DeployBatchExecutor.s.sol:DeployBatchExecutor --rpc-url https://arb-mainnet.g.alchemy.com/v2/-FDfJ1GYdKyvmVXVfQLTbr_7i04YGMKU --private-key $DEV_PRIVATE_KEY --broadcast -vv

deploy-executor-mainnet:
    cd contracts && forge script ./script/DeployBatchExecutor.s.sol:DeployBatchExecutor --rpc-url https://eth-mainnet.alchemyapi.io/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --private-key $DEV_PRIVATE_KEY --broadcast -vv
