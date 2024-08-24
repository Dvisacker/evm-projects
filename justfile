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
    uniswap_v2_factory_address=$(jq -r ".mainnet.uniswapv2.factory" $addressbook_path)
    uniswap_v2_router_address=$(jq -r ".mainnet.uniswapv2.router" $addressbook_path)
    weth_usdc_address=$(jq -r ".mainnet.uniswapv2.weth-usdc" $addressbook_path)


    echo "Downloading multicall from $multicall_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $multicall_address

    echo "Downloading uniswap V2 factory from $uniswap_v2_factory_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v2_factory_address

    echo "Downloading uniswap V2 router from $uniswap_v2_router_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v2_router_address

    # echo "Downloading uniswap V2 pool from $weth_usdc_address"
    # cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $weth_usdc_address

generate-bindings:
    #!/usr/bin/env bash
    bindings_path="./crates/strategies/uni-tri-arb/contracts/bindings"
    contract_root_path="./crates/strategies/uni-tri-arb/contracts/"
    rm -rf $bindings_path
    forge bind --bindings-path $bindings_path --root $contract_root_path --crate-name uni-tri-arb-bindings --force --skip-cargo-toml

build-bindings: download-contracts generate-bindings

fmt: 
    cargo +nightly fmt --all

clippy: 
    cargo clippy --all --all-features

