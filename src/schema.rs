// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    productions (product_id) {
        product_id -> Int4,
        product_title -> Text,
        product_image_url -> Text,
        product_price -> Int4,
        product_openprice -> Int4,
        product_tags -> Json,
        product_text -> Json,
        created_at -> Nullable<Timestamp>,
        product_thresholds -> Json,
        product_sold_status -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    productions,
);
