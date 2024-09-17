// @generated automatically by Diesel CLI.

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
    pools (id) {
        id -> Int4,
        address -> Varchar,
        chain -> Varchar,
        factory_address -> Varchar,
        exchange_name -> Varchar,
        exchange_type -> Varchar,
        token_a -> Varchar,
        token_a_symbol -> Varchar,
        token_a_decimals -> Int4,
        token_b -> Varchar,
        token_b_symbol -> Varchar,
        token_b_decimals -> Int4,
        reserve_0 -> Varchar,
        reserve_1 -> Varchar,
        fee -> Int4,
        filtered -> Nullable<Bool>,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(exchanges, pools, uni_v3_pools,);
