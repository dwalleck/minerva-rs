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
