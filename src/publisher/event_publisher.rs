use tokio::sync::mpsc;
use crate::orderbook::types::Event;
use kanal::Receiver;
pub struct EventPublisher{
    reciever : kanal::Receiver<Event>
}

impl EventPublisher {
    pub fn new(rx : kanal::Receiver<Event>)->Self{
        Self{
            reciever : rx
        }
        
    }
    pub fn start_publisher(&mut self){
        // ADD BATCH PUBLISHING HERE BEFORE 
        let mut count = 0u64;
        let mut last_log = std::time::Instant::now();
        loop {
            match self.reciever.recv() {
                Ok(_event)=>{
                    count += 1;
                }
                Err(_) => {
                    println!("[PUBLISHER] Channel closed, exiting");
                    break;
                }
            }
            if last_log.elapsed().as_secs() >= 5 {
                let rate = count as f64 / last_log.elapsed().as_secs_f64();
                eprintln!("[PUBLISHER RECEIVER] {:.2}M events/sec", rate / 1_000_000.0);
                count = 0;
                last_log = std::time::Instant::now();
            }
        }
    }
}