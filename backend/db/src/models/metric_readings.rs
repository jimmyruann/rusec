use crate::models::metric_sensors::MetricSensor;
use crate::schema::metric_readings;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, Debug, Clone,
)]
#[diesel(primary_key(metric_sensor_id, time))]
#[diesel(belongs_to(MetricSensor))]
#[diesel(table_name = metric_readings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MetricReading {
    pub metric_sensor_id: Uuid,
    pub time: NaiveDateTime,
    pub value: Decimal,
}
