set dotenv-load := true

#test contracts using forge
build-contracts:
    forge install --root ./contracts
    forge test --root ./contracts


#test contracts using forge
test-contracts: 
    forge test --root ./contracts


download-contracts:
    #!/usr/bin/env bash
    addressbook_path="./addressbook.json"
    multicall_address=$(jq -r ".mainnet.multicall" $addressbook_path)
    uniswap_v2_factory_address=$(jq -r ".mainnet.exchanges.univ2.uniswapv2.factory" $addressbook_path)
    uniswap_v2_router_address=$(jq -r ".mainnet.exchanges.univ2.uniswapv2.router" $addressbook_path)
    # weth_usdc_address=$(jq -r ".mainnet.univ2.uniswapv2.weth-usdc" $addressbook_path)
    uniswap_v3_factory_address=$(jq -r ".mainnet.exchanges.univ3.uniswapv3.factory" $addressbook_path)

    echo "Downloading multicall from $multicall_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $multicall_address

    echo "Downloading uniswap V2 factory from $uniswap_v2_factory_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v2_factory_address

    echo "Downloading uniswap V2 router from $uniswap_v2_router_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v2_router_address

    echo "Downloading uniswap V3 factory from $uniswap_v3_factory_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v3_factory_address

    # echo "Downloading uniswap V2 pool from $weth_usdc_address"
    # cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $weth_usdc_address

generate-bindings:
    #!/usr/bin/env bash
    bindings_path="./crates/bindings"
    contract_root_path="./crates/strategies/uni-tri-arb/contracts/"
    rm -rf $bindings_path
    forge bind --bindings-path $bindings_path --root $contract_root_path --crate-name bindings --force --skip-cargo-toml --alloy


generate-graphql-bindings:
    #!/usr/bin/env bash
    query_path="./crates/clients/graph/query.graphql"
    schema_path="./crates/clients/graph/schema.graphql"
    bindings_path="./crates/clients/graph/src"
    graphql-client generate $query_path --schema-path $schema_path --output-directory $bindings_path

build-bindings: download-contracts generate-bindings generate-graphql-bindings

fmt: 
    cargo +nightly fmt --all

clippy: 
    cargo clippy --all --all-features

setup-db:
    diesel setup --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

revert-db:
    diesel migration revert --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

migrate-db:
    diesel migration run --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

print-db:
    diesel print-schema -- --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml


run-bot-arbitrum:
    cargo run --bin bot -- --chain-id 42161

run-bot-mainnet:
    cargo run --bin bot -- --chain-id 1

get-filtered-pools:
    cargo run --bin cli -- filter --chain-id 42161 --checkpoint-path ./checkpoints/filtered-pools-arbitrum.json

get-filtered-pools-mainnet:
    cargo run --bin cli -- filter --chain-id 1 --checkpoint-path ./checkpoints/filtered-pools-mainnet.json
