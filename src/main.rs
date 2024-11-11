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
fn main() {}
