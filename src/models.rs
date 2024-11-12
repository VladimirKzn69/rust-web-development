// use diesel::Insertable;
extern crate diesel;

use crate::schema::*;
use diesel::{Insertable, Identifiable, Queryable};
use serde::{Serialize, Deserialize};
// use shoe_store::schema::variants;

#[macro_use]
// extern crate diesel;

use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use diesel::ExpressionMethods;
// use ::shoe_store::models::*;
use diesel::Connection;
use diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;
use crate::schema::products_variants;

/*/
use diesel::no_arg_sql_function;
no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);
*/
use diesel::define_sql_function;
use diesel::sql_types::Integer;

define_sql_function!{
    fn last_insert_rowid() -> Integer;
}



#[derive(Insertable, Debug)]
#[diesel(table_name=products)]
pub struct NewProduct {
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

// #[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[derive(Identifiable, Queryable, Debug)]
#[table_name = "variants"]
pub struct Variant {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name="variants"]
pub struct NewVariant {
    pub name: String,
}

// use shoe_store::schema::products_variants;

#[derive(Insertable, Debug)]
#[table_name="products_variants"]
pub struct NewProductVariant {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>
}
#[derive(Clone)]
pub struct NewVariantValue {
    pub variant: NewVariant,
    pub values: Vec<Option<String>>
}
pub struct NewCompleteProduct {
    pub product: NewProduct,
    pub variants: Vec<NewVariantValue>
}

// use super::models::NewProduct;
/*
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel::RunQueryDsl;
*/

/* pub fn create_product(new_product: NewProduct, conn: &mut SqliteConnection) -> Result<usize, Error>  {
    use crate::schema::products::dsl::*;
    // use crate::models::products::dsl::products;
    diesel::insert_into(products)
        .values(new_product)
        .execute(conn)
}
*/
pub fn create_product(new_product: NewCompleteProduct, conn: &mut SqliteConnection) -> Result<i32>  {
    use crate::schema::products::dsl::products;
    use crate::schema::variants::dsl::*;
    use crate::schema::products_variants::dsl::*;
    use diesel::Connection;
    use anyhow::Result;

    conn.transaction(|conn| {
        diesel::insert_into(products)
            .values(new_product.product)
            .execute(conn)?;

        let last_product_id: i32 = diesel::select(last_insert_rowid()).first(conn)?;

        for new_variant in new_product.variants {
            let variants_result =
                variants
                    .filter(name.eq(&new_variant.variant.name))
                    .limit(1)
                    .load::<Variant>(conn)?;

            let last_variant_id: i32 =
                match variants_result.first() {
                    Some(variant) => variant.id,
                    None => {
                        diesel::insert_into(variants)
                            .values(name.eq(&new_variant.variant.name))
                            .execute(conn)?;

                        diesel::select(last_insert_rowid()).first(conn)?
                    }
                };

            for new_value in new_variant.values {
                diesel::insert_into(products_variants)
                    .values(
                        (
                            product_id.eq(last_product_id), 
                            variant_id.eq(last_variant_id),
                            value.eq(new_value), 
                        )
                    )
                    .execute(conn)?;
            }
        }
        Ok(last_product_id)
    })
}
