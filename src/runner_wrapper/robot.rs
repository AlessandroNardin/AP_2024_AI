use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{get_score, go};
use robotics_lib::interface::Direction::{Down, Up};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use crate::state::{Action, MyState};


pub struct MyRobot{
    robot:Robot,
    action_receiver:Rc<Receiver<Action>>,
    state_sender:Sender<MyState>
}

impl MyRobot{
    pub fn new(action_receiver:Rc<Receiver<Action>>, state_sender:Sender<MyState>) -> Self{
        let robot = Robot::new();
        MyRobot{
            robot,
            action_receiver,
            state_sender,
        }
    }

}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let action = self.action_receiver.recv().unwrap();
        match action {
            Action::None => {}
            Action::Up => {go(self,world,Up);}
            Action::Down => {go(self,world,Down);}
            Action::Left => {}
            Action::Right => {}
        }
        let score = get_score(world) as f64;
        let reward = f64::to_be_bytes(score);
        let new_state = MyState::new(reward,vec![Action::None]);
        self.state_sender.send(new_state).unwrap();
    }

    fn handle_event(&mut self, event: Event) {
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}
