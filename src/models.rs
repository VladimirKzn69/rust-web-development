use diesel::Insertable;
use crate::schema::*;

#[derive(Insertable, Debug)]
#[diesel(table_name=products)]
pub struct NewProduct {
    pub name: String,
    pub cost: f64,
    pub active: bool,
}
// use super::models::NewProduct;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel::RunQueryDsl;

pub fn create_product(new_product: NewProduct, conn: &mut SqliteConnection) -> Result<usize, Error>  {
    use crate::schema::products::dsl::*;
    // use crate::models::products::dsl::products;
    diesel::insert_into(products)
        .values(new_product)
        .execute(conn)
}
