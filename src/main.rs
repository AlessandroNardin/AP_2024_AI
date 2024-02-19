use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rurel::AgentTrainer;
use rurel::mdp::{Agent, State};
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;
use crate::agent::MyAgent;
use crate::runner_wrapper::RunnerWrapper;

pub(crate) mod agent;
pub(crate) mod runner_wrapper;
pub(crate) mod state;

const GENERATIONS:u32 = 1000;
const ITERATIONS_PER_GEN:u32 = 1000;
const DEBUG:bool = false;


static mut print_image:bool = false;

fn main() {
    let (control_sender,control_receiver) = mpsc::channel();
    let (action_sender,action_receiver) = mpsc::channel();
    let (state_sender,state_receiver) = mpsc::channel();
    let h = thread::spawn(|| {
        let mut wrapper = RunnerWrapper::new(control_receiver,Rc::new(action_receiver),state_sender);
        wrapper.start();
    });
    let mut agent = MyAgent::new(control_sender,action_sender,state_receiver);
    let mut trainer = AgentTrainer::new();
    println!("Training Started");
    for GENERATION in 0..GENERATIONS {
        agent.new_gen();
        println!("Training gen {GENERATION}");
        trainer.train(&mut agent,
                      &QLearning::new(0.2, 0.1, 0.),
                      &mut FixedIterations::new(ITERATIONS_PER_GEN),
                      &RandomExploration::new());
    }
    unsafe { print_image = true; }
    agent.new_gen();
    for i in 0..200 {
        let a = trainer.best_action(agent.current_state());
        match a {
            None => { println!("Best action not available");break}
            Some(action) => {
                println!("Action {i}: {:?}",action);
                agent.take_action(&action);
                sleep(Duration::from_millis(100));
            }
        }
    }
}
