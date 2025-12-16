// ==========================================
// tests_balance_manager2_queues.rs
// Communication via ArrayQueue (new architecture)
// ==========================================

#[cfg(test)]
mod tests_balance_manager2_queues {
    use super::*;
    use crate::balance_manager::my_balance_manager2::MyBalanceManager2;
    use crate::orderbook::order::{Order, Side};
    use crate::orderbook::types::{Fills, Fill, OrderId, BalanceManagerError};
    use crate::balance_manager::types::{BalanceQuery, HoldingsQuery};
    use std::sync::Arc;
    use crossbeam::queue::ArrayQueue;
    use crossbeam::channel::unbounded;

    const QUEUE_CAPACITY: usize = 32768;

    fn setup_queues() -> (
        Arc<ArrayQueue<Order>>, // shm → BM
        Arc<ArrayQueue<Order>>, // BM → engine
        Arc<ArrayQueue<Fills>>  // engine → BM
    ) {
        (
            Arc::new(ArrayQueue::new(QUEUE_CAPACITY)),
            Arc::new(ArrayQueue::new(QUEUE_CAPACITY)),
            Arc::new(ArrayQueue::new(QUEUE_CAPACITY))
        )
    }

    fn setup_balance_manager() -> (
        MyBalanceManager2,
        Arc<ArrayQueue<Order>>,
        Arc<ArrayQueue<Order>>,
        Arc<ArrayQueue<Fills>>,
    ) {
        let (order_tx, order_rx) = unbounded::<Order>();
        let (fill_tx, fill_rx) = unbounded::<Fills>();
        let (shm_ch_tx, shm_ch_rx) = unbounded::<Order>();
        let (bal_tx, bal_rx) = unbounded::<BalanceQuery>();
        let (hold_tx, hold_rx) = unbounded::<HoldingsQuery>();

        let (shm_q, engine_q, fill_q) = setup_queues();

        let mut bm = MyBalanceManager2::new(
            order_tx.clone(),
            fill_rx,
            shm_ch_rx,
            bal_rx,
            hold_rx,
            fill_q.clone(),
            shm_q.clone(),
            engine_q.clone()
        );

        bm.state.user_id_to_index.insert(10, 1);
        bm.state.user_id_to_index.insert(20, 2);
        bm.state.holdings[2].available_holdings[0] = 100;

        (bm, shm_q, engine_q, fill_q)
    }

    fn create_order(user: u64, id: OrderId, side: Side, order_type : u8,  qty: u32, px: u64, sym: u32) -> Order {
        Order {
            user_id: user,
            order_id: id,
            side,
            order_type,
            shares_qty: qty,
            price: px,
            timestamp: 0,
            next: None,
            prev: None,
            symbol: sym
        }
    }

    fn create_fill(
        maker: u64,
        taker: u64,
        maker_id: OrderId,
        taker_id: OrderId,
        taker_side: Side,
        price: u64,
        qty: u32,
        sym: u32
    ) -> Fill {
        Fill {
            price,
            quantity: qty,
            taker_order_id: taker_id,
            maker_order_id: maker_id,
            taker_side,
            maker_user_id: maker,
            taker_user_id: taker,
            symbol: sym,
        }
    }

    // ------------------------------  
    // Queue-driven tests begin here
    // ------------------------------

    #[test]
    fn test_queue_order_flow() {
        let (mut bm, shm_q, engine_q, _) = setup_balance_manager();

        let order = create_order(10, 1, Side::Bid, 10, 100, 0);
        shm_q.push(order).unwrap();

        // BM drains shm
        let recv = shm_q.pop().unwrap();
        bm.check_and_lock_funds(recv).unwrap();
        engine_q.push(recv).unwrap();

        // Engine receives result
        let eng_recv = engine_q.pop().unwrap();

        assert_eq!(eng_recv.order_id, 1);
        assert_eq!(bm.state.balances[1].reserved_balance, 1000);
    }

    #[test]
    fn test_queue_flow_with_fill() {
        let (mut bm, shm_q, engine_q, fill_q) = setup_balance_manager();

        // Buyer places order
        let order = create_order(10, 1, Side::Bid, 10, 100, 0);
        shm_q.push(order).unwrap();
        let recv = shm_q.pop().unwrap();
        bm.check_and_lock_funds(recv).unwrap();
        engine_q.push(recv).unwrap();

        // Seller fill arrives
        let fill = create_fill(20, 10, 2, 1, Side::Bid, 100, 10, 0);
        fill_q.push(Fills { fills: vec![fill] }).unwrap();

        // BM processes fill
        let recv_fill = fill_q.pop().unwrap();
        bm.update_balances_after_trade(recv_fill).unwrap();

        assert_eq!(bm.state.balances[1].reserved_balance, 0);
        assert_eq!(bm.state.holdings[1].available_holdings[0], 10);
    }

    #[test]
    fn test_queue_partial_fill() {
        let (mut bm, shm_q, engine_q, fill_q) = setup_balance_manager();

        // Reserve for buyer
        let order = create_order(10, 1, Side::Bid, 100, 50, 0);
        shm_q.push(order).unwrap();
        let recv = shm_q.pop().unwrap();
        bm.check_and_lock_funds(recv).unwrap();
        engine_q.push(recv).unwrap();

        assert_eq!(bm.state.balances[1].reserved_balance, 5000);

        // First partial fill
        let fill = create_fill(20, 10, 2, 1, Side::Bid, 50, 30, 0);
        fill_q.push(Fills { fills: vec![fill] }).unwrap();
        let f = fill_q.pop().unwrap();
        bm.update_balances_after_trade(f).unwrap();

        assert_eq!(bm.state.balances[1].reserved_balance, 3500);
        assert_eq!(bm.state.holdings[1].available_holdings[0], 30);
    }
}
