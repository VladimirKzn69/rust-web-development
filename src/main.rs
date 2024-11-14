use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;
use ::shoe_store::models::*;

fn list_products(conn: &mut SqliteConnection) -> Vec<Product> {
    use ::shoe_store::schema::products::dsl::*;
    products
        .limit(10)
        .load::<Product>(conn)
        .expect("Error loading products")
}

use diesel::result::Error;
use diesel::Connection;
use ::shoe_store::establish_connection_test;

#[test]
fn test_list_products() {
    use shoe_store::establish_connection_test;
    use diesel::Connection;
    use diesel::result::Error;

    let mut connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|conn| {
        create_product(NewProduct {
            name: "boots".to_string(),
            cost: 13.23,
            active: true
        }, conn);
        create_product(NewProduct {
            name: "high heels".to_string(),
            cost: 20.99,
            active: true
        }, conn);
        create_product(NewProduct {
            name: "running shoes".to_string(),
            cost: 10.99,
            active: true
        }, conn);

        assert_eq!(serde_json::to_string(&list_products(conn)).unwrap(), 
            serde_json::to_string(&vec![
                Product {
                    id: Some(1),
                    name: "boots".to_string(),
                    cost: 13.23,
                    active: true
                },
                Product {
                    id: Some(2),
                    name: "high heels".to_string(),
                    cost: 20.99,
                    active: true
                },
                Product {
                    id: Some(3),
                    name: "running shoes".to_string(),
                    cost: 10.99,
                    active: true
                }
            ]).unwrap());

        Ok(())

    });
}