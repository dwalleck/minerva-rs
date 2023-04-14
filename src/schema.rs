// @generated automatically by Diesel CLI.

diesel::table! {
    test_results (id) {
        id -> Uuid,
        duration -> Numeric,
        run_at -> Timestamptz,
        name -> Text,
        status -> Text,
        error_message -> Nullable<Text>,
        job_name -> Nullable<Text>,
    }
}

diesel::table! {
    test_summaries (id) {
        id -> Uuid,
        name -> Text,
        errors -> Int4,
        failures -> Int4,
        skipped -> Int4,
        tests -> Int4,
        time -> Numeric,
        timestamp -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    test_results,
    test_summaries,
);
