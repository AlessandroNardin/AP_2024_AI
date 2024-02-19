use rurel::mdp::State;

#[derive(Clone,Eq, PartialEq,Hash,Debug)]
pub enum Action{
    None,
    Up,
    Down,
    Left,
    Right,
    Look,
    Fish,
    Sell,
}
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct MyState{
    reward: [u8;8],
    actions:Vec<<MyState as State>::A>,
    row:usize,
    col:usize,
}


impl MyState {
    pub fn new(reward:[u8;8], actions:Vec<<MyState as State>::A>, row:usize, col:usize) -> Self {
        MyState{
            reward,
            actions,
            row,
            col
        }
    }
}

impl State for MyState{
    type A = Action;

    fn reward(&self) -> f64 {
        f64::from_be_bytes(self.reward)
    }

    fn actions(&self) -> Vec<Self::A> {
        self.actions.clone()
    }
}