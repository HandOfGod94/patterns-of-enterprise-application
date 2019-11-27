table! {
    contracts (id) {
        id -> Int8,
        product -> Nullable<Int8>,
        revenue -> Nullable<Numeric>,
        datesigned -> Nullable<Date>,
    }
}

table! {
    product (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
    }
}

table! {
    revenuerecognition (contract, recognizedon) {
        contract -> Int8,
        amount -> Nullable<Int8>,
        recognizedon -> Date,
    }
}

allow_tables_to_appear_in_same_query!(
    contracts,
    product,
    revenuerecognition,
);
