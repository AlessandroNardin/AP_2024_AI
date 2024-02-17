use std::sync::mpsc::{Receiver, Sender};
use rurel::mdp::{Agent, State};
use crate::state::{Action, MyState};
use crate::state::Action::{None};

pub struct MyAgent{
    current_state:MyState,
    control_sender:Sender<u8>,
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
        let new_state = self.state_reciever.recv().unwrap();
        self.current_state = new_state;
    }
}

impl MyAgent{
    pub fn new(control_sender:Sender<u8>, action_sender:Sender<Action>, state_reciever:Receiver<MyState>) -> Self {
        let current_state = MyState::new(0.0f64.to_be_bytes(), vec![]);
        let mut agent = MyAgent{
            current_state,
            control_sender,
            action_sender,
            state_reciever,
        };
        agent.new_gen();
        agent
    }
    pub fn new_gen(&mut self){
        self.control_sender.send(2).unwrap();
        self.action_sender.send(None).unwrap();
        let new_state = self.state_reciever.recv().unwrap();
        self.current_state = new_state;
    }
}