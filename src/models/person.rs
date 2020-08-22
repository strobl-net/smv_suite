use crate::schema::persons;
use chrono::NaiveDateTime;

#[derive(Queryable, GraphQLObject, Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tags: Vec<String>,
    pub added: NaiveDateTime,
    pub changed: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "persons"]
pub struct NewPerson<'a> {
    pub name: &'a str,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tags: &'a Vec<String>,
    pub added: NaiveDateTime,
    pub changed: Option<NaiveDateTime>,
}

impl<'a> NewPerson<'a> {
    pub fn from_input(input: &InputPerson) -> NewPerson {
        NewPerson {
            name: &input.name,
            email: input.email.clone(),
            phone: input.phone.clone(),
            tags: &input.tags,
            added: chrono::Utc::now().naive_utc(),
            changed: None,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct InputPerson {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tags: Vec<String>,
}

#[derive(AsChangeset, GraphQLInputObject)]
#[table_name = "persons"]
pub struct UpdatePerson {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tags: Option<Vec<String>>,
}
