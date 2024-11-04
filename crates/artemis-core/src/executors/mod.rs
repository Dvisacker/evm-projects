//! Executors are responsible for taking actions produced by strategies and
//! executing them in different domains. For example, an executor might take a
//! `SubmitTx` action and submit it to the mempool.

pub mod crosschain_executor;
pub mod mempool_executor;
/// This executor submits transactions to the public mempool.
pub mod sequence_executor;
