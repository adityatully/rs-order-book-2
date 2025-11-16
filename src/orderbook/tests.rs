#[cfg(test)]
// change and verify 
mod tests {
    use crate::orderbook::order_book::OrderBook;
    use crate::orderbook::order::{Order, Side};


    // Utility to quickly create orders
    fn order(id: u64, side: Side, price: u64, qty: u64) -> Order {
        Order {
            id,
            side,
            limit_price: price,
            shares_quantity: qty,
        }
    }

    /*───────────────────────────────────────────*
     *  INSERTIONS & BASIC BOOK STATE
     *───────────────────────────────────────────*/
    #[test]
    fn test_insertions_and_volume() {
        let mut book = OrderBook::new(1);

        book.insert_order(order(1, Side::Bid, 100, 10));
        book.insert_order(order(2, Side::Bid, 100, 20));
        book.insert_order(order(3, Side::Ask, 120, 15));

        // Check bid level 100
        let level = book.bidside.levels.get(&100).unwrap();
        assert_eq!(level.total_volume, 30);
        assert_eq!(level.orders.len(), 2);

        // FIFO correctness
        assert_eq!(level.orders[0].id, 1);
        assert_eq!(level.orders[1].id, 2);

        // Ask check
        let level = book.askside.levels.get(&120).unwrap();
        assert_eq!(level.total_volume, 15);
    }

    /*───────────────────────────────────────────*
     *  MARKET BUY HITS BEST ASK
     *───────────────────────────────────────────*/
    #[test]
    fn test_market_buy_consumes_asks() {
        let mut book = OrderBook::new(1);

        book.insert_order(order(1, Side::Ask, 100, 10));
        book.insert_order(order(2, Side::Ask, 101, 5));

        let result = book.match_market_order(order(99, Side::Bid, 0, 12)).unwrap();

        assert_eq!(result.fills.len(), 2);

        // Fill quantities
        assert_eq!(result.fills[0].quantity, 10); // full consumption of order 1
        assert_eq!(result.fills[1].quantity, 2);  // partial of order 2

        // Level 100 must be removed
        assert!(book.askside.levels.get(&100).is_none());

        // Level 101 must remain with 3
        let level = book.askside.levels.get(&101).unwrap();
        assert_eq!(level.total_volume, 3);
    }

    /*───────────────────────────────────────────*
     *  LIMIT BID MATCHES LOWER ASK
     *───────────────────────────────────────────*/
    #[test]
    fn test_limit_bid_crosses_ask() {
        let mut book = OrderBook::new(1);

        book.insert_order(order(1, Side::Ask, 100, 10));
        book.insert_order(order(2, Side::Ask, 101, 10));

        let result = book.match_bid(order(3, Side::Bid, 105, 15)).unwrap();

        assert_eq!(result.fills.len(), 2);
        assert_eq!(result.fills[0].quantity, 10); // matches 100
        assert_eq!(result.fills[1].quantity, 5);  // matches 101 partially

        // No resting remainder (order fully consumed)
        assert!(book.bidside.levels.get(&105).is_none());
    }

    /*───────────────────────────────────────────*
     *  FIFO GUARANTEE
     *───────────────────────────────────────────*/
    #[test]
    fn test_fifo_matching() {
        let mut book = OrderBook::new(1);

        book.insert_order(order(1, Side::Ask, 100, 5));
        book.insert_order(order(2, Side::Ask, 100, 5));
        book.insert_order(order(3, Side::Ask, 100, 5));

        let result = book.match_market_order(order(99, Side::Bid, 0, 12)).unwrap();

        assert_eq!(result.fills.len(), 3);

        assert_eq!(result.fills[0].maker_order_id, 1);
        assert_eq!(result.fills[1].maker_order_id, 2);
        assert_eq!(result.fills[2].maker_order_id, 3);

        // last order is partial
        assert_eq!(result.fills[2].quantity, 2);
    }

    /*───────────────────────────────────────────*
     *  TOTAL VOLUME INVARIANT
     *───────────────────────────────────────────*/
    #[test]
    fn test_total_volume_invariant() {
        let mut book = OrderBook::new(1);

        book.insert_order(order(1, Side::Ask, 100, 10));
        book.insert_order(order(2, Side::Ask, 100, 20));

        let level = book.askside.levels.get(&100).unwrap();
        let sum: u64 = level.orders.iter().map(|o| o.shares_quantity).sum();
        assert_eq!(sum, level.total_volume);

        // Perform match
        let _ = book.match_market_order(order(99, Side::Bid, 0, 25)).unwrap();

        if let Some(level) = book.askside.levels.get(&100) {
            let sum: u64 = level.orders.iter().map(|o| o.shares_quantity).sum();
            assert_eq!(sum, level.total_volume);
        }
    }

    /*───────────────────────────────────────────*
     *  PARTIAL FILL WITH RESTING ORDER
     *───────────────────────────────────────────*/
    #[test]
    fn test_resting_bid_after_partial_fill() {
        let mut book = OrderBook::new(1);

        book.insert_order(order(1, Side::Ask, 100, 10));

        // bid of 15 → consumes 10 and rests with 5
        let result = book.match_bid(order(2, Side::Bid, 100, 15)).unwrap();

        assert_eq!(result.fills.len(), 1);
        assert_eq!(result.fills[0].quantity, 10);

        // Remaining 5 should rest at price 100
        let level = book.bidside.levels.get(&100).unwrap();
        assert_eq!(level.total_volume, 5);
        assert_eq!(level.orders[0].shares_quantity, 5);
    }

    /*───────────────────────────────────────────*
     *  BEST PRICE CALCULATION CHECK
     *───────────────────────────────────────────*/
    #[test]
    fn test_best_price_updates() {
        let mut book = OrderBook::new(1);

        book.insert_order(order(1, Side::Ask, 100, 10));
        book.insert_order(order(2, Side::Ask, 90, 10));
        book.insert_order(order(3, Side::Ask, 110, 10));

        assert_eq!(book.askside.best_price(), Some(90));

        let _ = book.match_market_order(order(99, Side::Bid, 0, 10)).unwrap();
        assert_eq!(book.askside.best_price(), Some(100));
    }
}
