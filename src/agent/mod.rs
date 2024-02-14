use std::sync::mpsc::{Receiver, Sender};
use rurel::mdp::{Agent, State};
use crate::state::{Action, MyState};

struct MyAgent<'a>{
    current_state:MyState,
    control_sender:Sender<u8>,
    a_req_reciver:Receiver<u8>,
    action_sender:Sender<&'a Action>,
    state_reciever:Receiver<MyState>
}
impl<'a> Agent<MyState> for MyAgent<'a> {
    fn current_state(&self) -> &MyState {
        &self.current_state
    }

    fn take_action(&mut self, action: &<MyState as State>::A) {
        self.control_sender.send(1).unwrap();
        self.a_req_reciver.recv().unwrap();
        self.action_sender.send(action).unwrap();
        self.current_state = self.state_reciever.recv().unwrap();
    }
}