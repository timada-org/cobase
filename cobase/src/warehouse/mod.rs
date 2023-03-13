mod aggregate;
mod command;
mod event;

pub use command::*;

#[cfg(test)]
mod tests {
    use actix::Addr;
    use tokio::time::{sleep, Duration};
    use uuid::Uuid;

    use crate::{
        command::Command, query::Query, tests::create_context, warehouse::ImportDataCommand,
    };

    #[actix::test]
    async fn success_create_warehouse() {
        // let ctx = create_context("success_create_warehouse").await;
        // let cmd = ctx.extract::<Addr<Command>>();
        // let query = ctx.extract::<Addr<Query>>();
        // let user_id = Uuid::new_v4();

        // let id = cmd
        //     .send(crate::command::CommandInput {
        //         user_id: user_id.to_string(),
        //         input: CreateCommand {
        //             name: "Central park".to_owned(),
        //         },
        //     })
        //     .await
        //     .unwrap()
        //     .unwrap();

        // sleep(Duration::from_millis(300)).await;

        // let warehouses = query
        //     .send(ListWarehousesQuery {
        //         user_id: user_id.to_owned(),
        //     })
        //     .await
        //     .unwrap()
        //     .unwrap();

        // assert_eq!(
        //     warehouses,
        //     vec![projection::Warehouse {
        //         id,
        //         name: "Central park".to_owned(),
        //         user_id
        //     }]
        // );
    }
}
