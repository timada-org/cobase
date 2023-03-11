mod aggregate;
mod command;
mod event;
mod query;

pub mod projection;

pub use aggregate::Group;
pub use command::*;
pub use query::*;

#[cfg(test)]
mod tests {
    use actix::Addr;
    use tokio::time::{sleep, Duration};
    use uuid::Uuid;

    use crate::{
        command::Command,
        group::{projection, CreateCommand, ListGroupsQuery},
        query::Query,
        tests::create_context,
    };

    #[actix::test]
    async fn success_create_group() {
        let ctx = create_context("success_create_group").await;
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

        let groups = query
            .send(ListGroupsQuery {
                user_id: user_id.to_owned(),
            })
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            groups,
            vec![projection::Group {
                id,
                name: "Central park".to_owned(),
                user_id
            }]
        );
    }
}
