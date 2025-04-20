// @generated automatically by Diesel CLI.

diesel::table! {
    activities (user_id) {
        user_id -> Int8,
        activity_name -> Nullable<Text>,
        description -> Nullable<Text>,
        from -> Nullable<Timestamptz>,
        to -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    my_task (id) {
        id -> Int4,
        task_name -> Varchar,
        task_description -> Varchar,
    }
}

diesel::table! {
    pg_details (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        pg_owner_pid -> Varchar,
        name -> Varchar,
        address -> Varchar,
        current_stayed_persons -> Nullable<Int4>,
        services -> Nullable<Array<Nullable<Text>>>,
        contacts -> Nullable<Array<Nullable<Text>>>,
        status -> Nullable<Varchar>,
        images -> Nullable<Array<Nullable<Text>>>,
        pg_type -> Nullable<Array<Nullable<Text>>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        city -> Nullable<Varchar>,
        area -> Nullable<Varchar>,
        landmark -> Nullable<Varchar>,
        pincode -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
    }
}

diesel::table! {
    pg_floor (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        #[max_length = 256]
        pg_wing_pid -> Varchar,
        #[max_length = 256]
        pg_details_pid -> Varchar,
        #[max_length = 256]
        floor_name -> Varchar,
        room_count -> Nullable<Int4>,
        status -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pg_owners (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        profile_pic -> Nullable<Varchar>,
        contact -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 255]
        fcm_tokne -> Nullable<Varchar>,
    }
}

diesel::table! {
    pg_rents (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        pg_details_pid -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        tenant_id -> Nullable<Varchar>,
        #[max_length = 255]
        room_id -> Nullable<Varchar>,
        #[max_length = 255]
        wing_id -> Nullable<Varchar>,
        #[max_length = 255]
        payment_type -> Nullable<Varchar>,
        #[max_length = 255]
        room_no -> Nullable<Varchar>,
        #[max_length = 255]
        tenant_name -> Nullable<Varchar>,
        rent -> Nullable<Int4>,
        payment_month -> Nullable<Timestamp>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    pg_rooms (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        #[max_length = 256]
        pg_wing_pid -> Varchar,
        #[max_length = 256]
        pg_details_pid -> Varchar,
        floor_name -> Nullable<Varchar>,
        room_no -> Varchar,
        room_type -> Varchar,
        room_rent -> Nullable<Int4>,
        total_beds -> Nullable<Int4>,
        available_beds -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pg_wings (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        #[max_length = 256]
        wing_name -> Varchar,
        #[max_length = 256]
        pg_details_pid -> Varchar,
        floor_count -> Nullable<Int4>,
        status -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    share_contact_logs (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        #[max_length = 100]
        user_pid -> Varchar,
        #[max_length = 100]
        pg_pid -> Varchar,
        #[max_length = 100]
        pg_owner_pid -> Varchar,
        #[max_length = 100]
        contact -> Varchar,
        pg_name -> Nullable<Varchar>,
        pg_owner_name -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_notice_period (id) {
        id -> Int4,
        pid -> Nullable<Varchar>,
        #[max_length = 255]
        tenant_name -> Nullable<Varchar>,
        #[max_length = 255]
        tenant_pid -> Varchar,
        #[max_length = 255]
        room_no -> Nullable<Varchar>,
        #[max_length = 255]
        room_pid -> Varchar,
        #[max_length = 255]
        wing_name -> Nullable<Varchar>,
        #[max_length = 255]
        wing_pid -> Varchar,
        #[max_length = 255]
        pg_pid -> Varchar,
        is_notice_period_over -> Nullable<Bool>,
        np_from -> Timestamp,
        np_to -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int8,
        #[max_length = 40]
        user_name -> Nullable<Varchar>,
        #[max_length = 40]
        user_email -> Nullable<Varchar>,
        age -> Nullable<Int8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    my_task,
    pg_details,
    pg_floor,
    pg_owners,
    pg_rents,
    pg_rooms,
    pg_wings,
    share_contact_logs,
    user_notice_period,
    users,
);
