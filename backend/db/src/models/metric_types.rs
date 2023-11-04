use crate::models::metric_categories::MetricCategory;
use crate::schema::metric_types;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, Debug, Clone,
)]
#[diesel(belongs_to(MetricCategory))]
#[diesel(table_name = metric_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MetricType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metric_category_id: String,
}
