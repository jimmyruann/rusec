// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Uuid,
        system_info -> Jsonb,
        created_at -> Timestamp,
    }
}

diesel::table! {
    metric_categories (id) {
        #[max_length = 32]
        id -> Varchar,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    metric_readings (metric_sensor_id, time) {
        metric_sensor_id -> Uuid,
        time -> Timestamp,
        value -> Numeric,
    }
}

diesel::table! {
    metric_sensors (id) {
        id -> Uuid,
        #[max_length = 32]
        metric_type_id -> Varchar,
        #[max_length = 32]
        metric_category_id -> Varchar,
        agent_id -> Uuid,
    }
}

diesel::table! {
    metric_types (id) {
        #[max_length = 32]
        id -> Varchar,
        name -> Text,
        description -> Text,
        #[max_length = 32]
        metric_category_id -> Varchar,
    }
}

diesel::joinable!(metric_readings -> metric_sensors (metric_sensor_id));
diesel::joinable!(metric_sensors -> agents (agent_id));
diesel::joinable!(metric_sensors -> metric_categories (metric_category_id));
diesel::joinable!(metric_sensors -> metric_types (metric_type_id));
diesel::joinable!(metric_types -> metric_categories (metric_category_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    metric_categories,
    metric_readings,
    metric_sensors,
    metric_types,
);
