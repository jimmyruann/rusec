use crate::models::agents::Agent;
use crate::models::metric_categories::MetricCategory;
use crate::models::metric_types::MetricType;
use crate::schema::metric_sensors;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, Debug, Clone,
)]
#[diesel(belongs_to(MetricCategory))]
#[diesel(belongs_to(MetricType))]
#[diesel(belongs_to(Agent))]
#[diesel(table_name = metric_sensors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MetricSensor {
    pub id: Uuid,
    pub metric_type_id: String,
    pub metric_category_id: String,
    pub agent_id: Uuid,
}
