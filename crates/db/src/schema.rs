// @generated automatically by Diesel CLI.

diesel::table! {
    curve_pools (id) {
        id -> Int4,
        address -> Varchar,
        chain -> Varchar,
        token_a -> Varchar,
        token_a_decimals -> Int4,
        token_a_symbol -> Varchar,
        token_a_balance -> Varchar,
        token_b -> Varchar,
        token_b_decimals -> Int4,
        token_b_symbol -> Varchar,
        token_b_balance -> Varchar,
        token_c -> Nullable<Varchar>,
        token_c_decimals -> Nullable<Int4>,
        token_c_symbol -> Nullable<Varchar>,
        token_c_balance -> Nullable<Varchar>,
        token_d -> Nullable<Varchar>,
        token_d_decimals -> Nullable<Int4>,
        token_d_symbol -> Nullable<Varchar>,
        token_d_balance -> Nullable<Varchar>,
        exchange_name -> Nullable<Varchar>,
        exchange_type -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tag -> Nullable<Varchar>,
    }
}

diesel::table! {
    erc4626_vaults (id) {
        id -> Int4,
        address -> Varchar,
        chain -> Varchar,
        vault_token -> Varchar,
        vault_token_decimals -> Int4,
        vault_token_symbol -> Varchar,
        asset_token -> Varchar,
        asset_token_decimals -> Int4,
        asset_token_symbol -> Varchar,
        vault_reserve -> Varchar,
        asset_reserve -> Varchar,
        deposit_fee -> Int4,
        withdraw_fee -> Int4,
        exchange_name -> Nullable<Varchar>,
        exchange_type -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tag -> Nullable<Varchar>,
    }
}

diesel::table! {
    exchanges (id) {
        id -> Int4,
        chain -> Varchar,
        factory_address -> Nullable<Varchar>,
        exchange_name -> Varchar,
        exchange_type -> Varchar,
    }
}

diesel::table! {
    tags (name) {
        name -> Varchar,
    }
}

diesel::table! {
    uni_v2_pools (id) {
        id -> Int4,
        address -> Varchar,
        chain -> Varchar,
        token_a -> Varchar,
        token_a_symbol -> Varchar,
        token_a_decimals -> Int4,
        token_b -> Varchar,
        token_b_symbol -> Varchar,
        token_b_decimals -> Int4,
        reserve_0 -> Varchar,
        reserve_1 -> Varchar,
        fee -> Int4,
        exchange_name -> Nullable<Varchar>,
        exchange_type -> Nullable<Varchar>,
        factory_address -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tag -> Nullable<Varchar>,
    }
}

diesel::table! {
    uni_v3_pools (id) {
        id -> Int4,
        address -> Varchar,
        chain -> Varchar,
        token_a -> Varchar,
        token_a_decimals -> Int4,
        token_a_symbol -> Varchar,
        token_b -> Varchar,
        token_b_decimals -> Int4,
        token_b_symbol -> Varchar,
        liquidity -> Nullable<Varchar>,
        sqrt_price -> Nullable<Varchar>,
        fee -> Nullable<Int4>,
        tick -> Nullable<Int4>,
        tick_spacing -> Nullable<Int4>,
        tick_bitmap -> Nullable<Jsonb>,
        ticks -> Nullable<Jsonb>,
        exchange_name -> Nullable<Varchar>,
        exchange_type -> Nullable<Varchar>,
        factory_address -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tag -> Nullable<Varchar>,
    }
}

diesel::joinable!(curve_pools -> tags (tag));
diesel::joinable!(erc4626_vaults -> tags (tag));
diesel::joinable!(uni_v2_pools -> tags (tag));
diesel::joinable!(uni_v3_pools -> tags (tag));

diesel::allow_tables_to_appear_in_same_query!(
    curve_pools,
    erc4626_vaults,
    exchanges,
    tags,
    uni_v2_pools,
    uni_v3_pools,
);
