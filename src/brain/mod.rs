use std::sync::mpsc::Sender;
use rurel::mdp::{Agent, State};

pub fn start_brain(sender:Sender<i32>){
    sender.send(3454);
}

struct MyAgent{


}
impl Agent<MyState> for MyAgent {
    fn current_state(&self) -> &MyState {
        todo!()
    }

    fn take_action(&mut self, action: &<MyState as State>::A) {
        todo!()
    }
}

#[derive(Eq,PartialEq,Hash,Clone)]
struct MyState{

}
impl State for MyState{
    type A = ();

    fn reward(&self) -> f64 {
        todo!()
    }

    fn actions(&self) -> Vec<Self::A> {
        todo!()
    }
}