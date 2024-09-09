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

diesel::allow_tables_to_appear_in_same_query!(
    exchanges,
    pools,
);
