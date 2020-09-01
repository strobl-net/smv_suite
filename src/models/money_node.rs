use crate::schema::money_nodes;
use chrono::NaiveDateTime;
use crate::db::types::{Branch, Currency};

#[derive(Queryable, GraphQLObject, Debug)]
pub struct MoneyNode {
    pub id: i32,
    pub branch: Branch,
    pub change: i32,
    pub currency: Currency,
    pub added: NaiveDateTime,
    pub changed: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "money_nodes"]
pub struct NewMoneyNode {
    pub branch: Branch,
    pub change: i32,
    pub currency: Currency,
    pub added: NaiveDateTime,
    pub changed: Option<NaiveDateTime>,
}

impl NewMoneyNode {
    pub fn from_input(input: InputMoneyNode) -> Self {
        Self {
            branch: input.branch,
            change: input.change,
            currency: input.currency,
            added: chrono::Utc::now().naive_utc(),
            changed: None,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct InputMoneyNode {
    pub branch: Branch,
    pub change: i32,
    pub currency: Currency,
}

#[derive(AsChangeset, GraphQLInputObject)]
#[table_name = "money_nodes"]
pub struct UpdateMoneyNode {
    pub branch: Option<Branch>,
    pub change: Option<i32>,
    pub currency: Option<Currency>,
}