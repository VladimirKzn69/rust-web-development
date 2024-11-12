/*
use shoe_store::models::{create_product, NewProduct};

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;
}

#[test]
fn create_product_test() {
    use diesel::result::Error;
    use diesel::Connection;
    use shoe_store::establish_connection_test;

    let mut connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|conn| {
        let results = create_product(
            NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true,
            },
            conn,
        );
        assert_eq!(Ok(1), results);
        Ok(())
    });
}
*/

use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use serde::{Serialize, Deserialize};
use shoe_store::schema::products; // Подключение к схеме
// pub use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Debug)]
#[derive(serde::Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

pub fn list_products(conn: &mut SqliteConnection) -> Result<Vec<Product>, diesel::result::Error> {
    // Загружаем все продукты из таблицы
    products::table.load::<Product>(conn)
}


use ::shoe_store::establish_connection_test;
use diesel::result::Error;

#[test]

fn create_product_test() {
    use diesel::Connection;
    use shoe_store::models::create_product;
    use shoe_store::models::NewCompleteProduct;
    use shoe_store::models::NewProduct;
    use shoe_store::models::NewVariantValue;
    use shoe_store::models::NewVariant;

    let mut connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|conn| {
        create_product(NewCompleteProduct {
            product: NewProduct {
                name: "boots".to_string(),
                cost: 13.23,
                active: true
            },
            variants: vec![
                NewVariantValue {
                    variant: NewVariant {
                        name: "size".to_string()
                    },
                    values: vec![
                        Some(12.to_string()),
                        Some(14.to_string()),
                        Some(16.to_string()),
                        Some(18.to_string())
                    ]
                }
            ]
        }, conn).unwrap();

        assert_eq!(
            serde_json::to_string(&list_products(conn).unwrap()).unwrap(),
            serde_json::to_string(&vec![
                (
                    Product {
                        id: Some(1),
                        name: "boots".to_string(),
                        cost: 13.23,
                        active: true
                    },
                    vec![
                        (
                            Some(12.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(14.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(16.to_string()),
                            "size".to_string()
                        ),
                        (
                            Some(18.to_string()),
                            "size".to_string()
                        )
                    ]
                ),
            ]).unwrap()
        );

        Ok(())
    });
}
fn main() {}
