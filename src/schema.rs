// @generated automatically by Diesel CLI.

diesel::table! {
    address (id) {
        id -> Uuid,
        rank -> Nullable<Text>,
        name -> Nullable<Text>,
        symbol -> Nullable<Text>,
        slug -> Nullable<Text>,
        token_address -> Nullable<Text>,
    }
}

diesel::table! {
    symbol (id) {
        id -> Uuid,
        title -> Text,
        symb -> Text,
        cmc_rank -> Nullable<Text>,
        circulating_supply -> Nullable<Text>,
        total_supply -> Nullable<Text>,
        max_supply -> Nullable<Text>,
        price -> Nullable<Text>,
        volume_24 -> Nullable<Text>,
        percent_change_1 -> Nullable<Text>,
        percent_change_24 -> Nullable<Text>,
        percent_change_7 -> Nullable<Text>,
        market_cap -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    address,
    symbol,
);
