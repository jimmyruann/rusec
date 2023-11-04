use crate::schema::agents;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = agents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agent {
    pub id: Uuid,
    pub system_info: serde_json::Value,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = agents)]
pub struct NewAgent {
    pub system_info: serde_json::Value,
}

pub fn create_agent(value: &NewAgent, conn: &mut PgConnection) -> Agent {
    diesel::insert_into(agents::table)
        .values(value)
        .returning(Agent::as_returning())
        .get_result(conn)
        .expect("Unable to create new agent record.")
}
