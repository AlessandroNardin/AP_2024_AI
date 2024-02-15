use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use robotics_lib::runner::Runner;
use crate::runner_wrapper::robot::MyRobot;
use crate::state::{Action, MyState};

pub(crate) mod robot;

struct RunnerWrapper{
    runner: Runner,
    control_receiver:Receiver<u8>,
    req_a_sender:Sender<u8>,
    action_receiver:Rc<Receiver<Action>>,
    state_sender:Sender<MyState>
}

impl RunnerWrapper {
    fn start(&mut self){
        loop {
            let message = self.control_receiver.recv();
            match message {
                Ok(val) => {
                    match val {
                        1 => { self.runner.game_tick(); }
                        2 => { self.init_new_gen(); }
                        _ => {}
                    }
                }
                Err(_) => { break }
            }
        }
    }

    fn init_new_gen(&mut self){
        //let generator = ...
        let robot = MyRobot::new(self.req_a_sender.clone(),self.action_receiver.clone(),self.state_sender.clone());
        //let robot = MyRobot::new(...)
        //self.runner = Runner::new(robot,generator)
    }
}
