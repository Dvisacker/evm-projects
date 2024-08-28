use super::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::MigrationHarness;

fn setup_test_db() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    conn
}

#[test]
fn test_create_pool() {
    let mut conn = setup_test_db();
    let new_pool = NewPool {
        address: "0x1234567890123456789012345678901234567890",
        chain: "ethereum",
        factory_address: "0x0987654321098765432109876543210987654321",
        exchange_name: "UniswapV2",
        token_a: "0x1111111111111111111111111111111111111111",
        token_a_symbol: "TOKEN_A",
        token_a_decimals: 18,
        token_b: "0x2222222222222222222222222222222222222222",
        token_b_symbol: "TOKEN_B",
        token_b_decimals: 18,
        reserve_0: "1000000000000000000",
        reserve_1: "2000000000000000000",
        fee: 300,
    };

    let result = create_pool(&mut conn, &new_pool);
    assert!(result.is_ok());
    let pool = result.unwrap();
    assert_eq!(pool.address, new_pool.address);
}

#[test]
fn test_get_all_pools() {
    let mut conn = setup_test_db();
    // Create a few pools
    create_pool(&mut conn, &NewPool { /* ... */ }).unwrap();
    create_pool(&mut conn, &NewPool { /* ... */ }).unwrap();

    let result = get_all_pools(&mut conn);
    assert!(result.is_ok());
    let pools = result.unwrap();
    assert_eq!(pools.len(), 2);
}

#[test]
fn test_get_pool_by_address() {
    let mut conn = setup_test_db();
    let address = "0x1234567890123456789012345678901234567890";
    create_pool(&mut conn, &NewPool { address, /* ... */ }).unwrap();

    let result = get_pool_by_address(&mut conn, address);
    assert!(result.is_ok());
    let pool = result.unwrap();
    assert_eq!(pool.address, address);
}

#[test]
fn test_update_pool() {
    let mut conn = setup_test_db();
    let address = "0x1234567890123456789012345678901234567890";
    create_pool(
        &mut conn,
        &NewPool {
            address,
            chain: "ethereum", /* ... */
        },
    )
    .unwrap();

    let updated_pool = NewPool {
        address,
        chain: "arbitrum",
        /* ... */
    };

    let result = update_pool(&mut conn, address, &updated_pool);
    assert!(result.is_ok());
    let pool = result.unwrap();
    assert_eq!(pool.chain, "arbitrum");
}

#[test]
fn test_delete_pool() {
    let mut conn = setup_test_db();
    let address = "0x1234567890123456789012345678901234567890";
    create_pool(&mut conn, &NewPool { address, /* ... */ }).unwrap();

    let result = delete_pool(&mut conn, address);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let get_result = get_pool_by_address(&mut conn, address);
    assert!(matches!(get_result, Err(Error::NotFound)));
}

#[test]
fn test_get_pools_by_chain() {
    let mut conn = setup_test_db();
    create_pool(
        &mut conn,
        &NewPool {
            chain: "ethereum", /* ... */
        },
    )
    .unwrap();
    create_pool(
        &mut conn,
        &NewPool {
            chain: "arbitrum", /* ... */
        },
    )
    .unwrap();
    create_pool(
        &mut conn,
        &NewPool {
            chain: "ethereum", /* ... */
        },
    )
    .unwrap();

    let result = get_pools_by_chain(&mut conn, "ethereum");
    assert!(result.is_ok());
    let pools = result.unwrap();
    assert_eq!(pools.len(), 2);
    assert!(pools.iter().all(|p| p.chain == "ethereum"));
}

#[test]
fn test_get_pools_by_exchange() {
    let mut conn = setup_test_db();
    create_pool(
        &mut conn,
        &NewPool {
            exchange_name: "UniswapV2", /* ... */
        },
    )
    .unwrap();
    create_pool(
        &mut conn,
        &NewPool {
            exchange_name: "SushiSwap", /* ... */
        },
    )
    .unwrap();
    create_pool(
        &mut conn,
        &NewPool {
            exchange_name: "UniswapV2", /* ... */
        },
    )
    .unwrap();

    let result = get_pools_by_exchange(&mut conn, "UniswapV2");
    assert!(result.is_ok());
    let pools = result.unwrap();
    assert_eq!(pools.len(), 2);
    assert!(pools.iter().all(|p| p.exchange_name == "UniswapV2"));
}
