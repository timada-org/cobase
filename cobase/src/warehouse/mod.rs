mod aggregate;
mod command;
mod event;
mod service;

pub use command::*;

#[cfg(test)]
mod tests {
    use actix::Addr;
    use evento::{CommandError, PgEvento};
    use opendal::Operator;
    use serde_json::json;
    use uuid::Uuid;

    use crate::{command::Command, tests::create_context, warehouse::ImportDataCommand};

    use super::aggregate::Warehouse;
    use super::service::{get_import_data_path, read_import_data};

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
                            "id": 1,
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
                            "id": 3,
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
            CommandError::BadRequest("Missing field id at index 1".to_owned())
        );
    }

    #[actix::test]
    async fn success_import_data_to_warehouse() {
        let ctx = create_context("success_import_data_to_warehouse").await;
        let cmd = ctx.extract::<Addr<Command>>();
        let evento = ctx.extract::<PgEvento>();
        let op = ctx.extract::<Operator>();
        let user_1 = Uuid::new_v4();
        let user_2 = Uuid::new_v4();

        let data_0 = vec![
            serde_json::from_value(json!({
                "id": 1,
                "email": "john.doe@timada.co",
                "first_name": "john",
                "last_name": "doe"
            }))
            .unwrap(),
            serde_json::from_value(json!({
                "id": 2,
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
            read_import_data(&op, &warehouse.storage_paths[0])
                .await
                .unwrap(),
            data_0
        );

        let data_1 = vec![
            serde_json::from_value(json!({
                "id": 2,
                "email": "albert.dupont@timada.co",
                "first_name": "albert",
                "last_name": "dupont"
            }))
            .unwrap(),
            serde_json::from_value(json!({
                "id": 3,
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
            read_import_data(&op, &warehouse.storage_paths[0])
                .await
                .unwrap(),
            data_0
        );

        assert_eq!(
            read_import_data(&op, &warehouse.storage_paths[1])
                .await
                .unwrap(),
            data_1
        );

        let data_0 = vec![
            serde_json::from_value(json!({
                "id": 1,
                "email": "john.doe@gmail.com",
                "first_name": "john",
                "last_name": "doe"
            }))
            .unwrap(),
            serde_json::from_value(json!({
                "id": 2,
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
            read_import_data(&op, &warehouse.storage_paths[0])
                .await
                .unwrap(),
            data_0
        );
    }
}
