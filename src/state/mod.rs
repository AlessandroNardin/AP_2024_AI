use rurel::mdp::State;

pub enum Action{
    Up,
    Down,
    Left,
    Right
}
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct MyState{
    reward:f64,
    actions:Vec<Self::A>
}

impl State for MyState{
    type A = Action;

    fn reward(&self) -> f64 {
        self.reward
    }

    fn actions(&self) -> Vec<Self::A> {
        self.actions.clone()
    }
}