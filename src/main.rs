use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
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

const GENERATIONS:u32 = 10;
const ITERATIONS_PER_GEN:u32 = 100;

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
                      &QLearning::new(0.2, 0.01, 2.),
                      &mut FixedIterations::new(ITERATIONS_PER_GEN),
                      &RandomExploration::new());
    }
    agent.new_gen();
    for i in 0..100 {
        let a = trainer.best_action(agent.current_state());
        match a {
            None => { println!("Best action not available");}
            Some(action) => {
                println!("Action {i}: {:?}",action);
                agent.take_action(&action);
            }
        }
    }
}
