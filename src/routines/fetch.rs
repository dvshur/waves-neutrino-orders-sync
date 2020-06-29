use crate::orders::BondOrder;
// use futures::executor::block_on;
use reqwest::Client;
use tokio::sync::mpsc::{self, UnboundedReceiver};

const ORDER_PREFIXES: [&str; 10] = [
    "order_price_",
    "order_total_",
    "order_owner_",
    "order_height_",
    "order_status_",
    "debug_order_currentPrice_",
    "debug_order_roi_",
    "order_filled_total_",
    "order_prev_",
    "order_next_",
];

pub fn start<T: Into<UnboundedReceiver<String>>>(
    orders_per_batch: usize,
    order_ids: T,
) -> UnboundedReceiver<Result<BondOrder, String>> {
    let (orders_or_errors_tx, orders_or_errors_rx) = mpsc::unbounded_channel();
    let client = reqwest::Client::builder().build();

    // I think I need a mutex for that
    // let post_body_size = ORDER_PREFIXES.len() * orders_per_batch;
    // let mut buffer = Vec::with_capacity(orders_per_batch);

    tokio::spawn(async {
        while let Some(id) = order_ids.recv().await {
            let keys = keys_for_order(&id);

            // todo request one order

            // if buffer.len() < orders_per_batch {
            //     buffer.push(id);
            // } else {
            // }
        }
    });

    orders_or_errors_rx
}

#[derive(Debug, Clone)]
struct PostBody {
    pub keys: Vec<String>,
}

// what could go wrong?
// timeout
// network error
// order missing
// single key missing
// data corruption of some kind

// returns orders that have succeeded and ids that failed
// pub async fn fetch_bond_orders(&self, order_ids: &[&str]) -> (Vec<BondOrder>, &[&str]) {
//     // let mut batches = Vec::new();
//     // let mut iter = order_ids.iter();
//     let keys: Vec<String> = keys(order_ids);

//     println!("{:?}", keys.len());

//     for from in (0..keys.len()).step_by(self.post_batch_size) {
//         println!("{:?}", &keys[from..(from + 10)]);
//     }

//     // let keys_to_fetch: Vec
//     // processing 10 orders (100 state keys) per POST request

//     todo!()
// }

// could go wrong:
// incorrect batch size -> failfast
// pub async fn fetch_bond_orders(
//     &self,
//     order_ids: &[&str],
// ) -> Result<Vec<BondOrder>, Box<dyn Error>> {
//     // let mut batches = Vec::new();
//     // let mut iter = order_ids.iter();

//     let post_bodies = Self::prepare_post_bodies(&self.post_batch_size, order_ids: &[&str]);

//     println!("Post bodies: {:?}", post_bodies);

//     // let keys_to_fetch: Vec
//     // processing 10 orders (100 state keys) per POST request

//     todo!()
// }

fn prepare_post_bodies(post_batch_size: &usize, order_ids: &[&str]) -> Vec<PostBody> {
    let mut res = Vec::new();
    let keys: Vec<String> = keys(order_ids);

    for from in (0..keys.len()).step_by(*post_batch_size) {
        // last batch can be not a full one
        if (from + post_batch_size) >= keys.len() {
            res.push(PostBody {
                keys: keys[from..].to_vec(),
            });
        } else {
            res.push(PostBody {
                keys: keys[from..(from + post_batch_size)].to_vec(),
            });
        }
    }

    res
}

fn keys(order_ids: &[&str]) -> Vec<String> {
    order_ids.iter().flat_map(|id| keys_for_order(id)).collect()
}

fn keys_for_order(order_id: &str) -> Vec<String> {
    ORDER_PREFIXES
        .iter()
        .map(|key_prefix| String::from(*key_prefix) + order_id)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NODE_URL: &str = "https://nodes.waves.exchange";
    const ORDER_1: &str = "HjdLCbd3iVrpszntyEaw2t1MZFm3LgAct15aXsu6uJjg";
    const ORDER_2: &str = "HMhVhHfHJ8VCVvtDL3XEzuuxVVayTSy6cHw1BAbR8zGm";

    #[test]
    fn correctly_assembles_keys() {
        let keys = keys(&[ORDER_1, ORDER_2]);
        assert_eq!(
            &keys[0],
            "order_price_HjdLCbd3iVrpszntyEaw2t1MZFm3LgAct15aXsu6uJjg"
        );
        assert_eq!(keys.len(), ORDER_PREFIXES.len() * 2);
    }

    #[test]
    fn correctly_splits_keys_into_post_bodies() {
        // one big chunk
        let request_bodies = prepare_post_bodies(&100, &[ORDER_1, ORDER_2]);
        assert_eq!(request_bodies.len(), 1);
        assert_eq!(request_bodies[0].keys.len(), 20);

        // uneven: 15 by 5
        let request_bodies = prepare_post_bodies(&15, &[ORDER_1, ORDER_2]);
        assert_eq!(request_bodies.len(), 2);
        assert_eq!(request_bodies[0].keys.len(), 15);
        assert_eq!(request_bodies[1].keys.len(), 5);

        // split by 10
        let request_bodies = prepare_post_bodies(&10, &[ORDER_1, ORDER_2]);
        assert_eq!(request_bodies.len(), 2);
        assert_eq!(request_bodies[0].keys.len(), 10);

        // split by 5
        let request_bodies = prepare_post_bodies(&5, &[ORDER_1, ORDER_2]);
        assert_eq!(request_bodies.len(), 4);
        assert_eq!(request_bodies[0].keys.len(), 5);
    }

    // #[test]
    // fn fetches_an_order() {
    //     let repo = new(NODE_URL, 10);

    //     let order_1 = block_on(repo.fetch_bond_orders(&[ORDER_1, ORDER_2]))
    //         .unwrap()
    //         .into_iter()
    //         .next()
    //         .unwrap();

    //     assert_eq!(&order_1.id, ORDER_1);
    // }
}
