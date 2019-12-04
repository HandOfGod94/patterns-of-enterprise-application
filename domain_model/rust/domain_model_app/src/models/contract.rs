use diesel::pg::types::sql_types::Money;
use chrono::NaiveDate;
use diesel::{Queryable, Associations};

#[derive(Queryable, Associations)]
#[belongs_to(Revenue)]
pub struct Contract {
    revenue: i64,
    date_signed: NaiveDate
}