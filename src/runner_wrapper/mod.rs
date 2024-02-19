use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use robotics_lib::runner::Runner;
use robotics_lib::world::tile::Tile;
use tyrannousarust_rex_world_generator::WorldGenerator;
use crate::runner_wrapper::robot::MyRobot;
use crate::state::{Action, MyState};

pub(crate) mod robot;

pub struct RunnerWrapper{
    runner: Runner,
    control_receiver:Receiver<u8>,
    action_receiver:Rc<Receiver<Action>>,
    state_sender:Sender<MyState>
}

impl RunnerWrapper {
    pub fn start(&mut self){
        loop {
            let message = self.control_receiver.recv();
            match message {
                Ok(val) => {
                    match val {
                        1 => { self.runner.game_tick();}
                        2 => { self.init_new_gen(); }
                        _ => {}
                    }
                }
                Err(_) => { break }
            }
        }
    }

    fn init_new_gen(&mut self){
        let mut generator = WorldGenerator::new()
            .set_size(30)
            .set_seed(4444)
            .set_max_quantity(5)
            .set_int_dist(10)
            .set_natural_size(0.9)
            .set_artificial_density(0.0)
            .set_natural_density(0.7);


        let robot = MyRobot::new(self.action_receiver.clone(),self.state_sender.clone(),30);
        self.runner = Runner::new(Box::new(robot),&mut generator).unwrap();
        self.runner.game_tick();
    }

    pub fn new(control_receiver:Receiver<u8>, action_receiver:Rc<Receiver<Action>>, state_sender:Sender<MyState>) -> RunnerWrapper{
        let robot = MyRobot::new(action_receiver.clone(),state_sender.clone(),20);
        let mut generator = WorldGenerator::new()
            .set_size(30)
            .set_seed(4444)
            .set_max_quantity(5)
            .set_int_dist(10)
            .set_natural_size(0.9)
            .set_artificial_density(0.0)
            .set_natural_density(0.7);


        let runner = Runner::new(Box::new(robot),&mut generator).unwrap();
        RunnerWrapper{
            runner,
            control_receiver,
            action_receiver,
            state_sender
        }
    }
}
