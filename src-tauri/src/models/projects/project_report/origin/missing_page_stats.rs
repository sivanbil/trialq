use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::QueryableByName;

#[derive(QueryableByName, Debug)]
pub struct MissingPageStats {
    #[diesel(sql_type = Integer)]
    pub data_page_count: i32,

    #[diesel(sql_type = Integer)]
    pub gt_7: i32,

    #[diesel(sql_type = Integer)]
    pub gt_14: i32,
}
