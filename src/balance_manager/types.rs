
use tokio::sync::oneshot;


pub struct BalanceQuery {
    pub user_id: u64,
    pub response_sender: oneshot::Sender<BalanceResponse>,
}

#[derive(Debug, Clone)]
pub struct BalanceResponse {
    pub available_balance: u64,
    pub reserved_balance: u64,
}

pub struct HoldingsQuery {
    pub user_id: u64,
    pub symbol: u32,
    pub response_sender: oneshot::Sender<HoldingsResponse>,
}

#[derive(Debug, Clone)]
pub struct HoldingsResponse {
    pub available: u32,
    pub reserved: u32,
}
