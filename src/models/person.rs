use crate::schema::persons;
use chrono::NaiveDateTime;

#[derive(Queryable, GraphQLObject, Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tags: Vec<String>,
    pub date_added: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "persons"]
pub struct NewPerson<'a> {
    pub name: &'a str,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tags: &'a Vec<String>,
    pub date_added: NaiveDateTime,
}

impl<'a> NewPerson<'a> {
    pub fn from_input(input: &InputPerson) -> NewPerson {
        NewPerson {
            name: &input.name,
            email: input.email.clone(),
            phone: input.phone.clone(),
            tags: &input.tags,
            date_added: chrono::Utc::now().naive_utc()
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