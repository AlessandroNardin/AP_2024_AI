use mpsc::channel;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

mod brain;
mod environment;

fn main() {
    let (sender,reciever) = mpsc::channel();
    thread::spawn(move || brain::start_brain(sender));
    thread::spawn(move || environment::start_env(reciever));
}
