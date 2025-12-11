use std::sync::atomic::{AtomicU64};
use std::mem::MaybeUninit;


pub struct SPSCqueue<T>{
    pub producer_index : AtomicU64,
    _pad1: [u8; 56],
    pub consumer_index : AtomicU64,
    _pad2: [u8; 56],
    pub buffer : Box<[MaybeUninit<T>]>,
    pub capacity : usize ,
    pub mask : usize,
}


impl<T> SPSCqueue<T>{
    pub fn new(capacity : usize)->Self{

        assert!(capacity.is_power_of_two(), "capacity must be a power of 2");
        
        let mut v = Vec::with_capacity(capacity);
        for _ in 0 .. capacity{
            v.push(MaybeUninit::uninit());
        }
        let buffer = v.into_boxed_slice();
        Self{
            producer_index : AtomicU64::new(0),
            _pad1: [0; 56],
            consumer_index : AtomicU64::new(0),
            buffer : buffer ,
            _pad2: [0; 56],
            capacity , 
            mask : capacity -1
        }
    }
}

