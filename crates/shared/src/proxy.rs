use std::{error::Error, sync::Arc};

use alloy::{
    primitives::{Address, U160, U256},
    providers::Provider,
};
use tokio::task::JoinSet;

pub async fn get_implementation<P: Provider + 'static>(
    provider: Arc<P>,
    token: Address,
) -> Result<Option<Address>, Box<dyn Error>> {
    // adapted from: https://github.com/gnosis/evm-proxy-detection/blob/main/src/index.ts
    let eip_1967_logic_slot = U256::from_str_radix(
        "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
        16,
    )
    .unwrap();
    let eip_1967_beacon_slot = U256::from_str_radix(
        "0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50",
        16,
    )
    .unwrap();
    let open_zeppelin_implementation_slot = U256::from_str_radix(
        "0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3",
        16,
    )
    .unwrap();
    let eip_1822_logic_slot = U256::from_str_radix(
        "0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7",
        16,
    )
    .unwrap();

    let implementation_slots = vec![
        eip_1967_logic_slot,
        eip_1967_beacon_slot,
        open_zeppelin_implementation_slot,
        eip_1822_logic_slot,
    ];

    let mut set = JoinSet::new();

    for slot in implementation_slots {
        let _provider = provider.clone();
        let fut = tokio::spawn(async move { _provider.get_storage_at(token, slot).await });
        set.spawn(fut);
    }

    while let Some(res) = set.join_next().await {
        let out = res???;
        let implementation = Address::from(U160::from(out));
        if implementation != Address::ZERO {
            return Ok(Some(implementation));
        }
    }

    Ok(None)
}
