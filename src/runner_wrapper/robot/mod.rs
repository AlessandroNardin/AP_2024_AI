mod memory;

use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{discover_tiles, get_score, go, robot_map, robot_view};
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;
use crate::runner_wrapper::robot::memory::MyMemory;
use crate::state::{Action, MyState};


pub struct MyRobot{
    robot:Robot,
    memory:MyMemory,
    action_receiver:Rc<Receiver<Action>>,
    state_sender:Sender<MyState>,
    d_tiles:u32
}

impl MyRobot{
    pub fn new(action_receiver:Rc<Receiver<Action>>, state_sender:Sender<MyState>, world_size:usize) -> Self{
        let robot = Robot::new();
        MyRobot{
            robot,
            memory: MyMemory::new(world_size),
            action_receiver,
            state_sender,
            d_tiles:0
        }
    }

}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let action = self.action_receiver.recv().unwrap();
        let mut score = 0.0;
        match action {
            Action::None => {}
            Action::Up => {
                let outcome = go(self,world,Up);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(_) => {score -= 10.0;}
                }
            }
            Action::Down => {
                let outcome = go(self,world,Down);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(_) => {score -= 10.0;}
                }
            }
            Action::Left => {
                let outcome = go(self,world,Left);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(_) => {score -= 10.0;}
                }
            }
            Action::Right => {
                let outcome = go(self,world,Right);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(_) => {score -= 10.0;}
                }
            }
            _ => {}
        }

        let discovered = self.memory.discovered_number;
        score = score + discovered as f64;
        let reward = f64::to_be_bytes(score);
        let new_state = MyState::new(reward,vec![Action::Up,Action::Down,Action::Left,Action::Right]);
        self.state_sender.send(new_state).unwrap();
        self.memory.gen_img();
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

fn count_discovered(map: &Vec<Vec<Option<Tile>>>) -> i32 {
    let mut discovered = 0;
    map.iter().for_each(|row|{
            row.iter().for_each(|element|{
                match element {
                    None => {}
                    Some(_) => {discovered+=1}
                }
            })
        }
    );
    discovered
}
