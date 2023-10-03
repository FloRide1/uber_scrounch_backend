// @generated automatically by Diesel CLI.

diesel::table! {
    command_products (id) {
        id -> Int4,
        command_id -> Int4,
        product_id -> Int4,
        amount -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    commands (id) {
        id -> Int4,
        user_id -> Int4,
        location_id -> Int4,
        delivery_id -> Nullable<Int4>,
        confirmed -> Bool,
        delivered -> Bool,
        canceled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    deliveries (id) {
        id -> Int4,
        time -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        sma_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Varchar>,
        price -> Float8,
        stock -> Int4,
        image_url -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 254]
        email -> Varchar,
        admin -> Bool,
        banned -> Bool,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(command_products -> commands (command_id));
diesel::joinable!(command_products -> products (product_id));
diesel::joinable!(commands -> deliveries (delivery_id));
diesel::joinable!(commands -> locations (location_id));
diesel::joinable!(commands -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    command_products,
    commands,
    deliveries,
    locations,
    products,
    users,
);
