use crate::orderbook::order_book::OrderBook;
use crate::orderbook::order::{Order,Side};
#[cfg(test)]
mod tests {
    use super::*;

    fn new_order(order_id: u64, side: Side, qty: u32, price: u64, timestamp: u64, symbol: u32) -> Order {
        Order::new(order_id, side, qty, price, timestamp, symbol)
    }

    #[test]
    fn test_basic_limit_bid_and_ask_match() {
        let mut book = OrderBook::new(1);

        // Insert limit ask, should be unmatched
        let ask = new_order(1, Side::Ask, 100, 105, 1, 1);
        book.insert_order(ask);
        assert_eq!(book.askside.levels.get(&105).unwrap().get_total_volume(), 100);
        assert_eq!(book.get_best_ask(), Some(105));
        assert_eq!(book.get_best_bid(), None);

        // Insert aggressive bid that matches
        let mut bid = new_order(2, Side::Bid, 100, 105, 2, 1);
        let result = book.match_bid(&mut bid).unwrap();
        assert_eq!(result.fills.fills.len(), 1);
        assert_eq!(result.remaining_qty, 0);
        assert_eq!(book.get_best_ask(), None); // Book should be empty now
        assert_eq!(book.get_best_bid(), None);
    }

    #[test]
    fn test_partial_fill_then_resting_order() {
        let mut book = OrderBook::new(1);

        // One resting ask at 105
        let ask = new_order(10, Side::Ask, 100, 105, 10, 1);
        book.insert_order(ask);

        // Bid for 50 at 105 (partial fill)
        let mut bid = new_order(11, Side::Bid, 50, 105, 11, 1);
        let result = book.match_bid(&mut bid).unwrap();
        assert_eq!(result.fills.fills.len(), 1);
        assert_eq!(result.remaining_qty, 0);

        // Remaining 50 ask should still be in book
        let level = book.askside.levels.get(&105).unwrap();
        assert_eq!(level.get_total_volume(), 50);

        // Second bid for 100 at 105 (should fill the 50 and rest the rest)
        let mut bid2 = new_order(12, Side::Bid, 100, 105, 12, 1);
        let result2 = book.match_bid(&mut bid2).unwrap();
        assert_eq!(result2.fills.fills.len(), 1);
        assert_eq!(result2.remaining_qty, 50); // 50 rested
        // Bid book should now have the leftover bid
        assert_eq!(book.bidside.levels.get(&105).unwrap().get_total_volume(), 50);
    }

    #[test]
    fn test_market_order_drains_multiple_levels() {
        let mut book = OrderBook::new(1);
        // Resting asks at 105 (60) and 106 (50)
        book.insert_order(new_order(21, Side::Ask, 60, 105, 21, 1));
        book.insert_order(new_order(22, Side::Ask, 50, 106, 22, 1));

        let mut market_bid = new_order(23, Side::Bid, 90, 110, 23, 1);
        let result = book.match_market_order(&mut market_bid).unwrap();
        assert_eq!(result.fills.fills.len(), 2);
        assert_eq!(result.fills.fills[0].quantity, 60);
        assert_eq!(result.fills.fills[1].quantity, 30);
        assert_eq!(result.remaining_qty, 0);

        // 106 ask should still have 20 left
        let level = book.askside.levels.get(&106).unwrap();
        assert_eq!(level.get_total_volume(), 20);
        assert_eq!(book.askside.levels.len(), 1); // 105 level removed
    }

    #[test]
    fn test_resting_orders_and_cancellation() { 
        let mut book = OrderBook::new(1);
        // Rest 2 bids at 101 (order_id:10, order_id:11)
        book.insert_order(new_order(10, Side::Bid, 100, 101, 10, 1));
        book.insert_order(new_order(11, Side::Bid, 50, 101, 11, 1));
        // Rest one bid at 100
        book.insert_order(new_order(12, Side::Bid, 120, 100, 12, 1));
        // Assert orderbook depth
        assert_eq!(book.bidside.levels.get(&101).unwrap().get_total_volume(), 150);
        assert_eq!(book.bidside.levels.get(&100).unwrap().get_total_volume(), 120);
        // Now, cancel order 11
        book.cancel_order(11);
        assert_eq!(book.bidside.levels.get(&101).unwrap().get_total_volume(), 100);
        // Cancel order 10
        book.cancel_order(10);
        // Depth at 101 should be gone now
        assert!(book.bidside.levels.get(&101).is_none() || book.bidside.levels.get(&101).unwrap().get_total_volume() <= 0);
    }

    #[test]
    fn test_edge_case_fill_remaining_and_price_levels() {
        let mut book = OrderBook::new(2);
        // Resting asks at two levels
        book.insert_order(new_order(100, Side::Ask, 80, 200, 1, 2));
        book.insert_order(new_order(101, Side::Ask, 60, 201, 2, 2));
        // Bid fills all of 200
        let mut bid = new_order(102, Side::Bid, 100, 200, 3, 2);
        let result = book.match_bid(&mut bid).unwrap();
        assert_eq!(result.fills.fills.len(), 1);
        assert_eq!(result.fills.fills[0].quantity, 80); // Filled all of 200
        assert_eq!(result.remaining_qty, 20); // Rested at 200
        // Next bid fills remaining
        let mut bid2 = new_order(103, Side::Bid, 20, 200, 4, 2);
        let result2 = book.match_bid(&mut bid2).unwrap();
        assert_eq!(result2.fills.fills.len(), 1);
        assert_eq!(result2.fills.fills[0].quantity, 20);
        assert_eq!(result2.remaining_qty, 0);
    }

    #[test]
    fn test_cancel_nonexistent_and_reuse_slots() {
        let mut book = OrderBook::new(3);
        book.insert_order(new_order(201, Side::Bid, 100, 500, 1, 3));
        // Cancel order that doesn't exist (should not panic)
        book.manager.remove_order(9999);
        assert_eq!(book.manager.all_orders.len(), 1); // Only one order existed
        // Insert new order and make sure slot was reused after actual cancellation
        book.manager.remove_order(201);
        let idx_reuse = book.manager.insert_order(new_order(202, Side::Bid, 200, 500, 2, 3));
        assert_eq!(idx_reuse, 0); // Slot reused if capacity=1
    }
}
