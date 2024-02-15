use std::sync::mpsc::{Receiver, Sender};
use rurel::mdp::{Agent, State};
use crate::state::{Action, MyState};
use crate::state::Action::{None, Up};

struct MyAgent{
    current_state:MyState,
    control_sender:Sender<u8>,
    a_req_reciver:Receiver<u8>,
    action_sender:Sender<Action>,
    state_reciever:Receiver<MyState>
}
impl Agent<MyState> for MyAgent {
    fn current_state(&self) -> &MyState {
        &self.current_state
    }

    fn take_action(&mut self, action:&Action) {
        self.control_sender.send(1).unwrap();
        self.action_sender.send(action.clone()).unwrap();
        self.current_state = self.state_reciever.recv().unwrap();
    }
}

impl MyAgent{
    fn new_gen(&mut self){
        self.control_sender.send(1).unwrap();
        self.a_req_reciver.recv().unwrap();
        self.action_sender.send(None).unwrap();
        self.current_state = self.state_reciever.recv().unwrap();
    }

    fn initialize_new_gen(&mut self){
        self.control_sender.send(1).unwrap();
        self.current_state = self.state_reciever.recv().unwrap();
    }
}