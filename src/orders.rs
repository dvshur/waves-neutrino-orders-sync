#[derive(Debug, Copy, Clone)]
pub enum OrderStatus {
    New,
    Filled,
}

#[derive(Debug, Clone)]
pub struct BondOrder {
    id: String,
    height: u32,
    owner: String,

    status: OrderStatus,

    price: u32,

    total: f64,
    filled_total: f64,

    debug_price: Option<u32>,
    debug_roi: Option<u32>,

    prev_order_id: Option<String>,
    next_order_id: Option<String>,

    is_first: bool,
}
