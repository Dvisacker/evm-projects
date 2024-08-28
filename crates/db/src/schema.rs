diesel::table! {
    pools (id) {
        id -> Integer,
        address -> Text,
        chain -> Text,
        factory_address -> Text,
        exchange_name -> Text,
        token_a -> Text,
        token_a_symbol -> Text,
        token_a_decimals -> Integer,
        token_b -> Text,
        token_b_symbol -> Text,
        token_b_decimals -> Integer,
        reserve_0 -> Text,
        reserve_1 -> Text,
        fee -> Integer,
    }
}
