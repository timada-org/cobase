mod aggregate;
mod command;
mod event;
mod query;

pub mod projection;

pub use aggregate::Room;
pub use command::*;
pub use query::*;

#[cfg(test)]
mod tests {
    use actix::Addr;
    use tokio::time::{sleep, Duration};
    use uuid::Uuid;

    use crate::{
        command::Command,
        query::Query,
        room::{projection, CreateCommand, ListRoomsQuery},
        tests::create_context,
    };

    #[actix::test]
    async fn success_create_room() {
        let ctx = create_context("success_create_room").await;
        let cmd = ctx.extract::<Addr<Command>>();
        let query = ctx.extract::<Addr<Query>>();
        let user_id = Uuid::new_v4();

        let id = cmd
            .send(crate::command::CommandInput {
                user_id: user_id.to_string(),
                input: CreateCommand {
                    name: "Central park".to_owned(),
                },
            })
            .await
            .unwrap()
            .unwrap();

        sleep(Duration::from_millis(300)).await;

        let rooms = query
            .send(ListRoomsQuery {
                user_id: user_id.to_owned(),
            })
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            rooms,
            vec![projection::Room {
                id,
                name: "Central park".to_owned(),
                user_id
            }]
        );
    }
}
