use crate::db::persons;
use crate::graphql::Context;
use crate::models::person::{NewPerson, Person, UpdatePerson};
use diesel::PgConnection;
use juniper::{FieldError, FieldResult};

pub struct PersonQuery;
pub struct PersonMutation;

impl PersonQuery {
    pub fn all(ctx: &Context) -> FieldResult<Vec<Person>> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        match persons::all(conn) {
            Ok(persons) => Ok(persons),
            Err(err) => FieldResult::Err(FieldError::from(err)),
        }
    }

    pub fn by_id(ctx: &Context, id: i32) -> FieldResult<Option<Person>> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        match persons::by_id(conn, id) {
            Ok(person) => Ok(Some(person)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(err)),
            },
        }
    }
}

impl PersonMutation {
    pub fn new(ctx: &Context, person: NewPerson) -> FieldResult<Person> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        match persons::new(conn, person) {
            Ok(person) => Ok(person),
            Err(err) => FieldResult::Err(FieldError::from(err)),
        }
    }

    pub fn update(ctx: &Context, person: UpdatePerson, id: i32) -> FieldResult<Person> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        match persons::update(conn, person, id) {
            Ok(person) => Ok(person),
            Err(err) => FieldResult::Err(FieldError::from(err)),
        }
    }

    pub fn delete(ctx: &Context, id: i32) -> FieldResult<Person> {
        let conn: &PgConnection = &ctx.pool.get().unwrap();
        match persons::delete(conn, id) {
            Ok(person) => Ok(person),
            Err(err) => FieldResult::Err(FieldError::from(err)),
        }
    }
}
