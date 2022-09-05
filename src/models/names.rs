use crate::config::db::Pool;
use crate::schema::names::{self};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use mockall::predicate::*;
use mockall::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Deserialize)]
pub struct Name {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Clone)]
#[table_name = "names"]
pub struct NewNameDTO {
    pub name: String,
}

impl NewNameDTO {
    pub fn new() -> Self {
        NewNameDTO {
            name: "Name".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct NamesRepo {
    conn: Pool,
}

impl NamesRepo {
    pub fn new(pool: Pool) -> Self {
        Self { conn: pool }
    }
}

#[automock]
pub trait NamesRepoTrait {
    fn create(&self, new_name: NewNameDTO) -> Result<i32, diesel::result::Error>;
    fn get_all(&self) -> QueryResult<Vec<Name>>;
}

impl NamesRepoTrait for NamesRepo {
    fn create(&self, new_name: NewNameDTO) -> Result<i32, diesel::result::Error> {
        use crate::schema::names::id;
        diesel::insert_into(names::table)
            .values(new_name)
            .returning(id)
            .get_result(&self.conn.get().unwrap())
    }

    fn get_all(&self) -> QueryResult<Vec<Name>> {
        names::table.load::<Name>(&self.conn.get().unwrap())
    }
}
