use crate::orderbook::order::Order;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct OrderManager{
    pub all_orders : Vec<Option<Order>>,
    pub id_to_index : FxHashMap<u64  , usize>,
    pub free_list : Vec<usize>
}

impl OrderManager{
    pub fn new()->Self{
        Self{
            all_orders : Vec::with_capacity(1_000_000),
            id_to_index: FxHashMap::with_capacity_and_hasher(
                1_000_000,
                Default::default()
            ),
            free_list : Vec::new(),
        }
    }

    
}

