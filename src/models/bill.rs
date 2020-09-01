use crate::schema::bills;
use chrono::NaiveDateTime;
use crate::db::types::Currency;

#[derive(Queryable, GraphQLObject, Debug)]
pub struct Bill {
    pub id: i32,
    pub received: NaiveDateTime,
    pub processed: Option<NaiveDateTime>,
    pub products: Option<Vec<i32>>,
    pub responsible: Option<i32>, // User ID
    pub organisation: Option<i32>, // Organisation ID
    pub change: i32,
    pub currency: Currency,
    pub transaction: i32, // Transaction ID
    pub added: NaiveDateTime,
    pub changed: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "bills"]
pub struct NewBill {
    pub received: NaiveDateTime,
    pub processed: Option<NaiveDateTime>,
    pub products: Option<Vec<i32>>,
    pub responsible: Option<i32>, // User ID
    pub organisation: Option<i32>, // Organisation ID
    pub change: i32,
    pub currency: Currency,
    pub transaction: i32, // Transaction ID
    pub added: NaiveDateTime,
    pub changed: Option<NaiveDateTime>,
}

impl NewBill {
    pub fn from_input(input: InputBill) -> Self {
        Self {
            received: input.received,
            processed: input.processed,
            products: input.products,
            responsible: input.responsible,
            organisation: input.organisation,
            change: input.change,
            currency: input.currency,
            transaction: input.transaction,
            added: chrono::Utc::now().naive_utc(),
            changed: None,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct InputBill {
    pub received: NaiveDateTime,
    pub processed: Option<NaiveDateTime>,
    pub products: Option<Vec<i32>>,
    pub responsible: Option<i32>, // User ID
    pub organisation: Option<i32>, // Organisation ID
    pub change: i32,
    pub currency: Currency,
    pub transaction: i32, // Transaction ID
}

#[derive(AsChangeset, GraphQLInputObject)]
#[table_name = "bills"]
pub struct UpdateBill {
    pub received: Option<NaiveDateTime>,
    pub processed: Option<NaiveDateTime>,
    pub products: Option<Vec<i32>>,
    pub responsible: Option<i32>, // User ID
    pub organisation: Option<i32>, // Organisation ID
    pub change: Option<i32>,
    pub currency: Option<Currency>,
    pub transaction: Option<i32>, // Transaction ID
}