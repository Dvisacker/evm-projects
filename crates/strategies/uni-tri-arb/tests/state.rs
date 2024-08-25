use cfmms::dex::DexVariant;
use ethers::providers::{Http, Provider};
use std::sync::Arc;
use uni_tri_arb_strategy::state::PoolState;

// #[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_pool_state() {
    // Setup
    let rpc_url = std::env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set");
    let provider = Provider::<Http>::try_from(rpc_url).expect("Failed to create provider");
    let provider = Arc::new(provider);

    let pool_state = PoolState::new(provider.clone()).await;
    assert!(
        pool_state.pools.is_empty(),
        "New PoolState should have empty pools"
    );

    // Test PoolState::add_pools
    let test_pools = vec![
        (
            "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8"
                .parse()
                .unwrap(),
            DexVariant::UniswapV3,
        ),
        (
            "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
                .parse()
                .unwrap(),
            DexVariant::UniswapV3,
        ),
    ];

    for (address, dex_variant) in test_pools {
        let dex_variant = DexVariant::from(dex_variant);
        pool_state
            .add_pool(address, dex_variant)
            .await
            .expect("Failed to add pool");
    }

    // assert_eq!(
    //     pool_state.pools.len(),
    //     2,
    //     "PoolState should have 2 pools after adding"
    // );

    // for (address, _) in &test_pools {
    //     assert!(
    //         pool_state.pools.contains_key(address),
    //         "Added pool not found in PoolState"
    //     );
}

// Test PoolState::sync_pools
// pool_state.sync_pools().await.expect("Failed to sync pools");

// for pool in pool_state.pools.iter() {
//     let a = pool.value();
//     let reserves = a
//         .get_pool_data(provider.clone())
//         .await
//         .expect("Failed to get pool data");

//     reserves

//     // reserves.
//     //     assert!(
//     //         reserves.0 > 0 && reserves.1 > 0,
//     //         "Pool reserves should be non-zero after sync"
//     //     );
// }
