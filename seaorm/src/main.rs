mod customer;
mod item;
mod order;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use sea_orm::{entity::prelude::*, DatabaseBackend, MockDatabase};

    use crate::customer::Entity as CustomerEntity;
    use crate::{
        customer::Model as CustomerModel, item::Model as ItemModel, order::Model as OrderModel,
    };

    #[tokio::test]
    async fn test_find_item() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![CustomerModel {
                customer_id: 1,
                name: "XYZ商事".into(),
                address: "東京都港区六本木1-1-1".into(),
                phone_number: "03-1234-5678".into(),
            }]])
            .append_query_results([vec![ItemModel {
                item_id: 1,
                name: "信州りんご".into(),
                price: 300,
            }]])
            .append_query_results([vec![OrderModel {
                order_id: 1,
                ordered_at: DateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                customer_id: 1,
                item_id: 1,
                amount: 10,
            }]])
            .into_connection();

        assert_eq!(
            CustomerEntity::find_by_id(1).one(&db).await?,
            Some(CustomerModel {
                customer_id: 1,
                name: "XYZ商事".into(),
                address: "東京都港区六本木1-1-1".into(),
                phone_number: "03-1234-5678".into(),
            })
        );

        Ok(())
    }
}
