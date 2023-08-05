use std::time::Duration;

use save_world::save_world;
use tokio::{task, time};

use crate::config::AppState;

pub mod config;
mod save_world;

const MINUTE_WAIT: u64 = 5;

#[tokio::main]
async fn main() {
    let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(MINUTE_WAIT * 60));
        let mut app_state = AppState::load_from_env();
        let graph = app_state.to_graph().await.unwrap();

        let Some(world_id) = app_state.world_to_backup() else {
            return;
        };

        loop {
            interval.tick().await;
            match save_world(&world_id, &graph, &mut app_state).await {
                Ok(e_tag) => println!("e_tag of new file {e_tag}"),
                Err(e) => {
                    println!("{e:?}");
                    break;
                }
            }
        }
    });

    forever.await.unwrap();
}
