// @generated automatically by Diesel CLI.

diesel::table! {
    party (party_id) {
        party_id -> Uuid,
        #[max_length = 60]
        party_type_id -> Nullable<Varchar>,
        #[max_length = 60]
        external_id -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 60]
        status_id -> Nullable<Varchar>,
        created_date -> Nullable<Timestamp>,
        #[max_length = 255]
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamp>,
        #[max_length = 255]
        last_modified_by_user_login -> Nullable<Varchar>,
        is_unread -> Nullable<Bool>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        #[max_length = 255]
        party_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    person (party_id) {
        party_id -> Uuid,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        middle_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 1]
        gender -> Nullable<Bpchar>,
        birth_date -> Nullable<Date>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    status (status_id) {
        #[max_length = 100]
        status_id -> Varchar,
        #[max_length = 100]
        status_code -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_login (user_login_id) {
        #[max_length = 255]
        user_login_id -> Varchar,
        #[max_length = 60]
        current_password -> Nullable<Varchar>,
        #[max_length = 60]
        otp_secret -> Nullable<Varchar>,
        #[max_length = 512]
        client_token -> Nullable<Varchar>,
        password_hint -> Nullable<Text>,
        is_system -> Nullable<Bool>,
        enabled -> Nullable<Bool>,
        has_logged_out -> Nullable<Bool>,
        require_password_change -> Nullable<Bool>,
        disabled_date_time -> Nullable<Timestamp>,
        successive_failed_logins -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        otp_resend_number -> Nullable<Int4>,
        party_id -> Nullable<Uuid>,
    }
}

diesel::joinable!(person -> party (party_id));
diesel::joinable!(user_login -> party (party_id));

diesel::allow_tables_to_appear_in_same_query!(party, person, status, user_login,);
