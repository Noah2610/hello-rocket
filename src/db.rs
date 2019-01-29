use rocket_contrib::databases::diesel;

#[database("pg")]
pub struct DbConnection(diesel::PgConnection);
