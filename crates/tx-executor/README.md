# tx-executor

The tx-executor crate provides a flexible and extensible way to encode and execute atomic blockchain transactions through a single BatchExecutor contract. It allows chaining multiple transactions together without needing to redeploy any contracts.

This is a modified rust version of https://github.com/Rubilmax/executooor

## Features

- Atomic execution of multiple transactions through a single contract call
- Flexible encoding of arbitrary contract calls
- Easily integrate new defi protocols by extending encoder.rs
- Support for common DeFi operations like:
  - Token swaps
  - Lending/borrowing 
  - Flash loans
  - And more


### Examples

```rust
let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

let (success, receipt) = encoder
    .add_wrap_eth(weth, input_amount)
    .add_transfer_erc20(weth, executor_address, input_amount)
    .add_odos_swap(input_amount, weth, usdc, slippage)
    .await
    .exec()
    .await?;


let (success, receipt) = encoder
    .add_wrap_eth(weth, amount)
    .add_transfer_erc20(weth, executor_address, amount)
    .add_approve_erc20(weth, pool_address, amount)
    .add_aave_v3_supply(weth, amount)
    .exec()
    .await?;
```

See tests for more examples


