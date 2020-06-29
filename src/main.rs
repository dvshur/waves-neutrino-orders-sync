mod orders;
mod routines;

use mpsc::UnboundedReceiver;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // todo config

    // listen to new orders and push their ids to the stream
    let (retries_tx, retries) = mpsc::unbounded_channel();

    let order_ids = routines::listen::start();

    let order_ids_with_retries = order_ids.merge(retries);

    // fetch orders info from Node and send downstream
    let order_fetch_results = routines::fetch::start(10, order_ids_with_retries);

    let successfully_inserted = order_fetch_results.filter_map(|res| match res {
        Ok(v) => Some(v),
        Err(_) => None,
    });

    // insert orders info into database
    let insertion_results = routines::insert::start(successfully_inserted);

    // todo error handling
    tokio::spawn(async move {
        while let Some(msg) = successfully_inserted.recv().await {
            println!("{}", msg);
        }
    });

    Ok(())
}
