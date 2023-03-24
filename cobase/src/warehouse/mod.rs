mod aggregate;
mod command;
mod event;
mod query;
mod service;

pub mod projection;

pub use command::*;
pub use projection::{Warehouse, WarehouseData};
pub use query::*;

#[cfg(test)]
mod tests {
    use actix::Addr;
    use evento::{CommandError, PgEvento};
    use opendal::Operator;
    use serde_json::json;
    use tokio::time::{sleep, Duration};
    use uuid::Uuid;

    use crate::query::Query;
    use crate::{
        command::Command,
        tests::create_context,
        warehouse::{projection, ImportDataCommand, ListWarehouseDatasQuery},
    };

    use super::aggregate::Warehouse;
    use super::service::read_import_data;

    #[actix::test]
    async fn fail_missing_id_import_data_to_warehouse() {
        let ctx = create_context("fail_missing_id_import_data_to_warehouse").await;
        let cmd = ctx.extract::<Addr<Command>>();
        let user_1 = Uuid::new_v4();

        let err = cmd
            .send(crate::command::CommandInput {
                user_id: user_1.to_string(),
                input: ImportDataCommand {
                    data: vec![
                        serde_json::from_value(json!({
                            "_id": 1,
                            "email": "john.doe@timada.co",
                            "first_name": "john",
                            "last_name": "doe"
                        }))
                        .unwrap(),
                        serde_json::from_value(json!({
                            "email": "albert.dupont@timada.co",
                            "first_name": "albert",
                            "last_name": "not my last name"
                        }))
                        .unwrap(),
                        serde_json::from_value(json!({
                            "_id": 3,
                            "email": "lennie.rice@timada.co",
                            "first_name": "lennie",
                            "last_name": "rice"
                        }))
                        .unwrap(),
                    ],
                },
            })
            .await
            .unwrap()
            .unwrap_err();

        assert_eq!(
            err,
            CommandError::BadRequest(
                "Missing field _id or not (string | number) at index 1".to_owned()
            )
        );
    }

    #[actix::test]
    async fn success_import_data_to_warehouse() {
        let ctx = create_context("success_import_data_to_warehouse").await;
        let cmd = ctx.extract::<Addr<Command>>();
        let query = ctx.extract::<Addr<Query>>();
        let evento = ctx.extract::<PgEvento>();
        let op = ctx.extract::<Operator>();
        let user_1 = Uuid::new_v4();
        let user_2 = Uuid::new_v4();

        let data_0 = vec![
            serde_json::from_value(json!({
                "_id": 1,
                "email": "john.doe@timada.co",
                "first_name": "john",
                "last_name": "doe"
            }))
            .unwrap(),
            serde_json::from_value(json!({
                "_id": 2,
                "email": "albert.dupont@timada.co",
                "first_name": "albert",
                "last_name": "not my last name"
            }))
            .unwrap(),
        ];

        cmd.send(crate::command::CommandInput {
            user_id: user_1.to_string(),
            input: ImportDataCommand {
                data: data_0.clone(),
            },
        })
        .await
        .unwrap()
        .unwrap();

        let (warehouse, _) = evento
            .load::<Warehouse, _>(&user_1.to_string())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            read_import_data(op, &warehouse.storage_paths[0])
                .await
                .unwrap(),
            data_0
        );

        sleep(Duration::from_millis(300)).await;

        let warehouse_datas = query
            .send(ListWarehouseDatasQuery {
                user_id: user_1.to_owned(),
            })
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            warehouse_datas,
            vec![
                projection::WarehouseData {
                    id: warehouse_datas[0].id.to_owned(),
                    key: "1".to_owned(),
                    data: serde_json::to_value(&data_0[0]).unwrap(),
                    created_at: warehouse_datas[0].created_at.to_owned(),
                    updated_at: warehouse_datas[0].updated_at.to_owned(),
                },
                projection::WarehouseData {
                    id: warehouse_datas[1].id.to_owned(),
                    key: "2".to_owned(),
                    data: serde_json::to_value(&data_0[1]).unwrap(),
                    created_at: warehouse_datas[1].created_at.to_owned(),
                    updated_at: warehouse_datas[1].updated_at.to_owned(),
                }
            ]
        );

        assert!(warehouse_datas[0].updated_at.is_none());
        assert!(warehouse_datas[1].updated_at.is_none());

        let data_1 = vec![
            serde_json::from_value(json!({
                "_id": 2,
                "email": "albert.dupont@timada.co",
                "first_name": "albert",
                "last_name": "dupont"
            }))
            .unwrap(),
            serde_json::from_value(json!({
                "_id": 3,
                "email": "lennie.rice@timada.co",
                "first_name": "lennie",
                "last_name": "rice"
            }))
            .unwrap(),
        ];

        cmd.send(crate::command::CommandInput {
            user_id: user_1.to_string(),
            input: ImportDataCommand {
                data: data_1.clone(),
            },
        })
        .await
        .unwrap()
        .unwrap();

        let (warehouse, _) = evento
            .load::<Warehouse, _>(&user_1.to_string())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            read_import_data(op, &warehouse.storage_paths[0])
                .await
                .unwrap(),
            data_0
        );

        assert_eq!(
            read_import_data(op, &warehouse.storage_paths[1])
                .await
                .unwrap(),
            data_1
        );

        sleep(Duration::from_millis(300)).await;

        let warehouse_datas = query
            .send(ListWarehouseDatasQuery {
                user_id: user_1.to_owned(),
            })
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            warehouse_datas,
            vec![
                projection::WarehouseData {
                    id: warehouse_datas[0].id.to_owned(),
                    key: "1".to_owned(),
                    data: serde_json::to_value(&data_0[0]).unwrap(),
                    created_at: warehouse_datas[0].created_at.to_owned(),
                    updated_at: warehouse_datas[0].updated_at.to_owned(),
                },
                projection::WarehouseData {
                    id: warehouse_datas[1].id.to_owned(),
                    key: "2".to_owned(),
                    data: serde_json::to_value(&data_1[0]).unwrap(),
                    created_at: warehouse_datas[1].created_at.to_owned(),
                    updated_at: warehouse_datas[1].updated_at.to_owned(),
                },
                projection::WarehouseData {
                    id: warehouse_datas[2].id.to_owned(),
                    key: "3".to_owned(),
                    data: serde_json::to_value(&data_1[1]).unwrap(),
                    created_at: warehouse_datas[2].created_at.to_owned(),
                    updated_at: warehouse_datas[2].updated_at.to_owned(),
                }
            ]
        );

        assert!(warehouse_datas[0].updated_at.is_none());
        assert!(warehouse_datas[1].updated_at.is_some());
        assert!(warehouse_datas[2].updated_at.is_none());

        let data_0 = vec![
            serde_json::from_value(json!({
                "_id": 1,
                "email": "john.doe@gmail.com",
                "first_name": "john",
                "last_name": "doe"
            }))
            .unwrap(),
            serde_json::from_value(json!({
                "_id": 2,
                "email": "albert.dupont@gmail.com",
                "first_name": "albert",
                "last_name": "dupont"
            }))
            .unwrap(),
        ];

        cmd.send(crate::command::CommandInput {
            user_id: user_2.to_string(),
            input: ImportDataCommand {
                data: data_0.clone(),
            },
        })
        .await
        .unwrap()
        .unwrap();

        let (warehouse, _) = evento
            .load::<Warehouse, _>(&user_2.to_string())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            read_import_data(op, &warehouse.storage_paths[0])
                .await
                .unwrap(),
            data_0
        );

        sleep(Duration::from_millis(300)).await;

        let warehouse_datas = query
            .send(ListWarehouseDatasQuery {
                user_id: user_2.to_owned(),
            })
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            warehouse_datas,
            vec![
                projection::WarehouseData {
                    id: warehouse_datas[0].id.to_owned(),
                    key: "1".to_owned(),
                    data: serde_json::to_value(&data_0[0]).unwrap(),
                    created_at: warehouse_datas[0].created_at.to_owned(),
                    updated_at: warehouse_datas[0].updated_at.to_owned(),
                },
                projection::WarehouseData {
                    id: warehouse_datas[1].id.to_owned(),
                    key: "2".to_owned(),
                    data: serde_json::to_value(&data_0[1]).unwrap(),
                    created_at: warehouse_datas[1].created_at.to_owned(),
                    updated_at: warehouse_datas[1].updated_at.to_owned(),
                },
            ]
        );
    }
}
