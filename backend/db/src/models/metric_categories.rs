use diesel::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::metric_categories;

lazy_static! {
    static ref ID_VALIDATION_REGEX: Regex = Regex::new(r"^[a-z_]$").unwrap();
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = metric_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MetricCategory {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Insertable, Validate, Debug)]
#[diesel(table_name = metric_categories)]
pub struct NewMetricCategory {
    #[validate(
        length(min = 3),
        regex(
            path = "ID_VALIDATION_REGEX",
            message = "Metric Category ID can only contain lower case letters and underscore."
        )
    )]
    pub id: String,
    #[validate(length(min = 3))]
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Validate, AsChangeset, Debug)]
#[diesel(table_name = metric_categories)]
pub struct UpdateMetricCategory {
    #[validate(length(min = 3))]
    pub name: String,
    pub description: String,
}

pub fn create_metric_category(
    value: &NewMetricCategory,
    conn: &mut PgConnection,
) -> MetricCategory {
    diesel::insert_into(metric_categories::table)
        .values(value)
        .returning(MetricCategory::as_returning())
        .get_result(conn)
        .expect("Unable to save metric category.")
}

pub fn find_metric_category(id: &str, conn: &mut PgConnection) -> Option<MetricCategory> {
    metric_categories::table
        .select(MetricCategory::as_select())
        .filter(metric_categories::id.eq(id))
        .first(conn)
        .optional()
        .expect("Unable to fetch metric category.")
}

pub fn find_metric_categories(limit: Option<i64>, conn: &mut PgConnection) -> Vec<MetricCategory> {
    let row_limit = limit.unwrap_or(20);
    metric_categories::table
        .limit(row_limit)
        .load(conn)
        .expect("Unable to get metric categories.")
}

pub fn check_metric_category_exist(id: &str, conn: &mut PgConnection) -> bool {
    let c: i64 = metric_categories::table
        .filter(metric_categories::id.eq(id))
        .count()
        .first::<i64>(conn)
        .expect("Unable to get metric category count");

    return c > 0;
}

pub fn update_metric_category(
    id: &str,
    values: &UpdateMetricCategory,
    conn: &mut PgConnection,
) -> Result<MetricCategory, String> {
    let exist = check_metric_category_exist(id, conn);

    if !exist {
        return Err(format!("Metric category ID `{}` does not exist.", id));
    }

    let data = diesel::update(metric_categories::table)
        .filter(metric_categories::id.eq(id))
        .set(values)
        .returning(MetricCategory::as_returning())
        .get_result(conn)
        .expect(format!("Unable to update metric category `{}`", id).as_str());

    return Ok(data);
}
