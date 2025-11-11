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
        while let Some(_event) = self.reciever.recv().ok() {
            // publish to kafka or our message broker 
            //println!("event recied {:?}" ,event);
        }
    }
}