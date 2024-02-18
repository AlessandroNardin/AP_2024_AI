mod memory;

use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use fish_by_ifrustrati::tool::fish;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{discover_tiles, get_score, go, robot_map, robot_view};
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::utils::go_allowed;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::Fish;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;
use crate::print_image;
use crate::runner_wrapper::robot::memory::MyMemory;
use crate::state::{Action, MyState};


pub struct MyRobot{
    robot:Robot,
    action_receiver:Rc<Receiver<Action>>,
    state_sender:Sender<MyState>,
    memory:MyMemory,
    last_disc:i32,
}

impl MyRobot{
    pub fn new(action_receiver:Rc<Receiver<Action>>, state_sender:Sender<MyState>, world_size:usize) -> Self{
        let robot = Robot::new();
        MyRobot{
            robot,
            action_receiver,
            state_sender,
            memory: MyMemory::new(world_size),
            last_disc: 0,
        }
    }

}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        //MANAGE ACTION
        let action = self.action_receiver.recv().unwrap();
        match action {
            Action::None => {}
            Action::Up => {
                let outcome = go(self,world,Up);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(val) => {}
                }
            }
            Action::Down => {
                let outcome = go(self,world,Down);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(val) => {}
                }
            }
            Action::Left => {
                let outcome = go(self,world,Left);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(val) => {}
                }
            }
            Action::Right => {
                let outcome = go(self,world,Right);
                match outcome {
                    Ok(view) => { self.memory.discover_view(view.0,view.1.0,view.1.1)}
                    Err(val) => {}
                }
            }
            Action::Fish => {
                let outcome = fish(self, world, Down, 5);
            }
            _ => {}
        }

        //COMPUTE CURRENT REWARD
        let discovered = self.memory.discovered_number;
        let disc_now = discovered - self.last_disc;
        self.last_disc = discovered;

        let reward = disc_now as i32;
        let reward_bytes = f64::to_be_bytes(reward as f64);

        //COMPUTE POSSIBLE ACTIONS
        let mut action_vector = Vec::new();

        if let Ok(()) = go_allowed(self,world,&Up) { action_vector.push(Action::Up) }
        if let Ok(()) = go_allowed(self,world,&Down) { action_vector.push(Action::Down) }
        if let Ok(()) = go_allowed(self,world,&Left) { action_vector.push(Action::Left) }
        if let Ok(()) = go_allowed(self,world,&Right) { action_vector.push(Action::Right) }

        //GENERATE AND SEND STATE
        let new_state = MyState::new(reward_bytes,action_vector);
        self.state_sender.send(new_state).unwrap();
        unsafe {
            if print_image {
                self.memory.gen_img();
            }
        }
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
