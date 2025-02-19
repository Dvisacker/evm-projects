set dotenv-load := true

#test contracts using forge
build-contracts:
    forge install --root ./contracts
    forge test --root ./contracts

build-amm-contracts:
    forge compile --root ./crates/amms/contracts

#test contracts using forge
test-contracts: 
    forge test --root ./contracts

generate-amm-bindings:
    forge bind \
    --bindings-path ./crates/amms/src/bindings \
    --root ./crates/amms/contracts \
    --remappings "forge-std/=lib/forge-std/src/" \
    --remappings "@uniswap/v3-core/=lib/v3-core/" \
    --remappings "@aperture/=lib/uni-v3-lib/src/" \
    --remappings "solady/=lib/solady/" \
    --module --alloy --alloy-version v0.11.1 --overwrite

generate-shared-bindings:
    forge bind --bindings-path ./crates/shared/src/bindings --root ./crates/shared/contracts --module --alloy --alloy-version v0.11.1 --overwrite
    forge bind --bindings-path ./crates/types/src/bindings --root ./crates/types/contracts --module --alloy --alloy-version v0.11.1 --overwrite

generate-executor-binding:
    #!/usr/bin/env bash
    bindings_path="./crates/tx-executor/src/bindings"
    contract_root_path="./crates/tx-executor/contracts"
    rm -rf $bindings_path
    forge bind --bindings-path $bindings_path --root $contract_root_path --module --alloy --alloy-version v0.11.1 --via-ir --overwrite

generate-simulator-bindings:
    forge bind --bindings-path ./crates/tx-simulator/src/bindings --root ./crates/tx-simulator/contracts --module --alloy --alloy-version v0.11.1 --via-ir --overwrite

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

run-aerodrome-bot:
    RUST_BACKTRACE=full cargo run --bin bot -- --chain-id 8453 --strategy base-arb

get-filtered-pools CHAIN_ID:
    cargo run --bin cli -- filter --chain-id {{CHAIN_ID}}

get-uni-v3-pools CHAIN_ID EXCHANGE_NAME STEP START_BLOCK:
    cargo run --bin cli -- get-uniswap-v3-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}} --from-block {{START_BLOCK}} --step {{STEP}}

get-uni-v2-pools CHAIN_ID EXCHANGE_NAME:
    cargo run --bin cli -- get-uniswap-v2-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}}

get-aerodrome-pools:
    cargo run --bin cli -- get-aerodrome-pools

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
    cd crates/tx-executor/contracts && forge script ./script/DeployBatchExecutor.s.sol:DeployBatchExecutor --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast -vv

deploy-simulator-local:
    cd crates/tx-simulator/contracts && forge script ./script/DeployTxSimulator.s.sol:DeployTxSimulator --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast -vv

deploy-executor-base:
    cd crates/tx-executor/contracts && forge script ./script/DeployBatchExecutor.s.sol:DeployBatchExecutor --rpc-url https://base-mainnet.g.alchemy.com/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --private-key $DEV_PRIVATE_KEY --broadcast -vv

deploy-simulator-base:
    cd crates/tx-simulator/contracts && forge script ./script/DeployTxSimulator.s.sol:DeployTxSimulator --rpc-url https://base-mainnet.g.alchemy.com/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --private-key $DEV_PRIVATE_KEY --broadcast -vv

deploy-executor-arbitrum:
    cd crates/tx-executor/contracts && forge script ./script/DeployBatchExecutor.s.sol:DeployBatchExecutor --rpc-url https://arb-mainnet.g.alchemy.com/v2/-FDfJ1GYdKyvmVXVfQLTbr_7i04YGMKU --private-key $DEV_PRIVATE_KEY --broadcast -vv

deploy-simulator-arbitrum:
    cd crates/tx-simulator/contracts && forge script ./script/DeployTxSimulator.s.sol:DeployTxSimulator --rpc-url https://arb-mainnet.g.alchemy.com/v2/-FDfJ1GYdKyvmVXVfQLTbr_7i04YGMKU --private-key $DEV_PRIVATE_KEY --broadcast -vv

deploy-executor-mainnet:
    cd crates/tx-executor/contracts && forge script ./script/DeployBatchExecutor.s.sol:DeployBatchExecutor --rpc-url https://eth-mainnet.alchemyapi.io/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --private-key $DEV_PRIVATE_KEY --broadcast -vv

deploy-simulator-mainnet:
    cd crates/tx-simulator/contracts && forge script ./script/DeployTxSimulator.s.sol:DeployTxSimulator --rpc-url https://eth-mainnet.alchemyapi.io/v2/fVddI-_ivqrBOeXVNVF2uqSvzZfSgwrw --private-key $DEV_PRIVATE_KEY --broadcast -vv

