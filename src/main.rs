use mpsc::channel;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use rurel::AgentTrainer;
use rurel::mdp::Agent;
use crate::agent::MyAgent;
use crate::runner_wrapper::RunnerWrapper;
use crate::state::Action::{Down, Up};

pub(crate) mod agent;
pub(crate) mod runner_wrapper;
pub(crate) mod state;

fn main() {
    let (control_sender,control_receiver) = mpsc::channel();
    let (action_sender,action_receiver) = mpsc::channel();
    let (state_sender,state_receiver) = mpsc::channel();
    let h = thread::spawn(|| {
        let mut wrapper = RunnerWrapper::new(control_receiver,Rc::new(action_receiver),state_sender);
        wrapper.start();
    });
    let mut agent = MyAgent::new(control_sender,action_sender,state_receiver);
    loop {
        agent.take_action(&Up);
        agent.take_action(&Down);
    }
}
