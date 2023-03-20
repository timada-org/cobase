mod aggregate;
mod command;
mod event;

pub use command::*;

#[cfg(test)]
mod tests {
    use actix::Addr;
    use evento::{CommandError, PgEvento};
    use serde_json::json;
    use uuid::Uuid;

    use crate::{command::Command, tests::create_context, warehouse::ImportDataCommand};

    use super::aggregate::Warehouse;

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
            CommandError::BadRequest("missing field id at index 1".to_owned())
        );
    }

    #[actix::test]
    async fn success_import_data_to_warehouse() {
        let ctx = create_context("success_import_data_to_warehouse").await;
        let cmd = ctx.extract::<Addr<Command>>();
        let evento = ctx.extract::<PgEvento>();
        let user_1 = Uuid::new_v4();
        let user_2 = Uuid::new_v4();

        cmd.send(crate::command::CommandInput {
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
                        "id": 2,
                        "email": "albert.dupont@timada.co",
                        "first_name": "albert",
                        "last_name": "not my last name"
                    }))
                    .unwrap(),
                ],
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
            warehouse,
            Warehouse {
                storage_paths: vec![format!("import-data/{}_0.tid", user_1.to_string())]
            }
        );

        // TODO: check if exist in storage

        cmd.send(crate::command::CommandInput {
            user_id: user_1.to_string(),
            input: ImportDataCommand {
                data: vec![
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
                ],
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
            warehouse,
            Warehouse {
                storage_paths: vec![
                    format!("import-data/{}_0.tid", user_1.to_string()),
                    format!("import-data/{}_1.tid", user_1.to_string())
                ]
            }
        );

        // TODO: check if exist in storage

        cmd.send(crate::command::CommandInput {
            user_id: user_2.to_string(),
            input: ImportDataCommand {
                data: vec![
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
                ],
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
            warehouse,
            Warehouse {
                storage_paths: vec![format!("import-data/{}_0.tid", user_2.to_string())]
            }
        );

        // TODO: check if exist in storage
    }
}
