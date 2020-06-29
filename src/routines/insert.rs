use crate::orders::BondOrder;
use tokio::sync::mpsc::UnboundedReceiver;

// todo typed errors
pub fn start(orders: UnboundedReceiver<BondOrder>) -> UnboundedReceiver<Result<String, String>> {
    todo!()
}
