use crate::orderbook::price_level::PriceLevel;
use crate::orderbook::order::Side;
use std::collections::BTreeMap;



// this is what we wanted to produce 
// first step , decide what stream do you want to produce each time when next function is called on you iterator 
#[derive(Debug)]
pub struct LevelInfo{
    pub price : u64 , 
    pub qty : u32 , 
    pub cumalative_depth : u32
}


// the derived iterators are wrappers over the standard iteratros , the standard iterator we are using is the iterator 
// of a Btree Map , the Btree map is storing u64 , PirceLevel so in the derived iterator we would get that iter 
//and along side of it we add some fields what we want which will be derived from the base iterator 
// now Btree map stores in ascending but asks and bids wud have diff order so we perform runtime polymorp9hsim 
// using dynamic dispath we wrap it into box dyn wich has the iterator trait of the Item u64 , Pirclevel
// this means that iter can be any type that exhiits the Iterator trait and yeilds the Item u64 , Pircelevel 
// because that is what we will get on iterating the BtreeMap 

pub struct LevelsWithCumalativeDepth<'a>{
    iter : Box<dyn Iterator<Item = (&'a u64, &'a PriceLevel)> + 'a>,
    cumalative_depth : u32
}


// constructor for our iterator , here we give out data structure whoose base iterator we wud use as an input and the side 
// depending on the side we dynamically dispath the iterator , either iter or iter.rev. cummaltive dept 0
impl<'a> LevelsWithCumalativeDepth<'a>{
    // iterator of a BtreeMap takes an immutable refrence only 
    // the constructor says , i take in input a BtreeMap which livs for the lifetime a and returns iterator which also lived 
    // for the life tike a and that iterator has the entires which also live for the life time a 
    pub fn new(price_level_map : &'a BTreeMap<u64 , PriceLevel>,side : Side)->Self{
        let iter : Box<dyn Iterator<Item = (&'a u64, &'a PriceLevel)> +'a> = match side {
            Side::Ask => Box::new(price_level_map.iter().rev()),
            Side::Bid => Box::new(price_level_map.iter())
        };
        Self{
            iter , 
            cumalative_depth : 0 
        }
    }
}
// we implement the Iterator trait 
impl<'a> Iterator for LevelsWithCumalativeDepth<'a>{
    type Item = LevelInfo ;
    fn next(&mut self)->Option<Self::Item>{
        self.iter.next().map(|entry|{
            let price = *entry.0;
            let quantity = entry.1.get_total_volume();
            self.cumalative_depth = self.cumalative_depth.saturating_add(quantity);

            LevelInfo{
                price , qty : quantity , cumalative_depth : self.cumalative_depth
            }
        })
    }
}



// 4 basic steps 1. define what we want to get when we call next 
// define the iterator struct which takes a pre defined iterator as an input and the additional field 
// define the constructor and the Iterator trait which returns the type Item which we decided in the first step


// the .iter methof in the BtreeMap takes a mutable refrence and then we use the entry of the Map in a struct so we 
// were rquired the lIfe time 