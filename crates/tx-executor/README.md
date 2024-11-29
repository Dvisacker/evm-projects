# tx-executor

The tx-executor crate provides a flexible and extensible way to encode and execute atomic blockchain transactions through a single BatchExecutor contract. It allows chaining multiple transactions together without needing to redeploy any contracts.

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

See tests for examples of how to use the encoder


