extern crate dotenv;

use diesel::prelude::*;
use juniper::{EmptySubscription, RootNode, graphql_object};

use crate::db::PgPool;

mod gratitude;
use gratitude::*;

#[derive(Clone)]
pub struct Context {
    pub dbpool: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    fn item_by_id(context: &Context, item_id: i32) -> Gratitude {
        use crate::schema::gratitude::dsl::*;
        let connection = context.dbpool.get().unwrap();

        let resp = gratitude
            .filter(id.eq(item_id))
            .first::<Gratitude>(&connection)
            .expect("Error could not load");

        return resp;
    }

    fn all_items(context: &Context) -> Vec<Gratitude> {
        use crate::schema::gratitude::dsl::*;
        let connection = context.dbpool.get().unwrap();

        gratitude
            .load::<Gratitude>(&connection)
            .expect("Error could not load")
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    fn new_job(context: &Context, data: NewGratitude) -> Gratitude {
        let connection = context.dbpool.get().unwrap();

        let res = diesel::insert_into(crate::schema::gratitude::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving new job");

        return res;
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
