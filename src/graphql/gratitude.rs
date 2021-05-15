use chrono::prelude::*;

use crate::schema::gratitude;

#[derive(Queryable, Clone)]
pub struct Gratitude {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub item: String,
}

#[juniper::graphql_object(description = "Queries the gratitude")]
impl Gratitude {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    pub fn item(&self) -> &str {
		&self.item
	}
}

#[derive(juniper::GraphQLInputObject, Insertable, AsChangeset)]
#[table_name = "gratitude"]
pub struct NewGratitude {
    pub item: String,
}
