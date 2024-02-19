mod memory;

use std::cmp::max;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use fish_by_ifrustrati::tool::fish;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{ go, robot_view};
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::utils::go_allowed;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::{Coin, Fish, Market};
use robotics_lib::world::tile::{Content, Tile, TileType};
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
    water_found:bool,
    market_found:bool,
    last_action:Action,
    last_fish:usize,
    last_coin:usize,
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
            water_found:false,
            market_found:false,
            last_action:Action::None,
            last_fish:0,
            last_coin:0,
        }
    }

}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let action = self.action_receiver.recv().unwrap();

        //MANAGE ACTION
        match action {
            Action::None => {}
            Action::Up => {
                let _ = go(self,world,Up);
                self.last_action = Action::Down;
            }
            Action::Down => {
                let _ = go(self,world,Down);
                self.last_action = Action::Up;
            }
            Action::Left => {
                let _ = go(self,world,Left);
                self.last_action = Action::Right;
            }
            Action::Right => {
                let _ = go(self,world,Right);
                self.last_action = Action::Left;
            }
            Action::Fish => {
                let _ = fish(self, world, Down, 5);
                self.last_action = Action::Fish;
            }
            Action::Sell => {
                match self.get_backpack().get_contents().get(&Fish(0)) {
                    None => {}
                    Some(_) => {
                        let _ = swift_seller::SwiftSeller::swift_seller(self,world,vec![Fish(0)]);
                        self.last_action = Action::Sell;
                    }
                }
            }
            _ => {}
        }
        let view = robot_view(self,world);
        let row = self.robot.coordinate.get_row();
        let col = self.robot.coordinate.get_col();
        self.memory.discover_view(&view,row,col);


        //NEW_TILES
        let disc = self.memory.discovered_number;
        let disc_now = disc - self.last_disc;
        self.last_disc = disc;

        //FISH
        let fish = match self.get_backpack().get_contents().get(&Fish(0)) {
            None => {0}
            Some(quantity) => {*quantity}
        };
        let fish_now = max(0,fish as i32 - self.last_fish as i32) as usize;
        self.last_fish = fish;

        //COIN
        let coin = match self.get_backpack().get_contents().get(&Coin(0)) {
            None => {0}
            Some(quantity) => {*quantity}
        };
        let coin_now = max(0,coin as i32 - self.last_coin as i32) as usize;
        self.last_coin = coin;

        let mut reward = disc_now + 2 * fish_now as i32 + 4 * coin as i32;

        if self.water_found != self.memory.water_found { reward += 100; }
        if self.market_found != self.memory.market_found { reward += 100; }
        self.water_found = self.memory.water_found;
        self.market_found = self.memory.market_found;

        //COMPUTE POSSIBLE ACTIONS
        let mut action_vector = Vec::new();

        if let Ok(()) = go_allowed(self,world,&Up) { if Action::Up != self.last_action {action_vector.push(Action::Up)} }
        if let Ok(()) = go_allowed(self,world,&Down) { if Action::Down != self.last_action {action_vector.push(Action::Down)} }
        if let Ok(()) = go_allowed(self,world,&Left) { if Action::Left != self.last_action {action_vector.push(Action::Left)} }
        if let Ok(()) = go_allowed(self,world,&Right) { if Action::Right != self.last_action {action_vector.push(Action::Right)} }
        if near_fish(&view) { action_vector.push(Action::Fish)}
        if near_market(&view) {action_vector.push(Action::Sell)}

        //GENERATE AND SEND STATE
        let row = self.robot.coordinate.get_row();
        let col = self.robot.coordinate.get_col();

        let reward_bytes = f64::to_be_bytes(reward as f64);
        let new_state = MyState::new(reward_bytes,action_vector,row,col);
        self.state_sender.send(new_state).unwrap();
        unsafe {
            if print_image {
                self.memory.gen_img();
                println!("{fish}-{coin}");
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

fn near_market(view:&Vec<Vec<Option<Tile>>>) -> bool {
    match &view[0][1] {
        None => {}
        Some(tile) => {if let Market(_)=tile.content {return true;}}
    }
    match &view[1][0] {
        None => {}
        Some(tile) => {if let Market(_)=tile.content {return true;}}
    }
    match &view[1][2] {
        None => {}
        Some(tile) => {if let Market(_)=tile.content {return true;}}
    }
    match &view[2][1] {
        None => {}
        Some(tile) => {if let Market(_)=tile.content {return true;}}
    }
    false
}
fn near_fish(view:&Vec<Vec<Option<Tile>>>) -> bool {
    match &view[0][1] {
        None => {}
        Some(tile) => {if let Fish(_)=tile.content {return true;}}
    }
    match &view[1][0] {
        None => {}
        Some(tile) => {if let Fish(_)=tile.content {return true;}}
    }
    match &view[1][2] {
        None => {}
        Some(tile) => {if let Fish(_)=tile.content {return true;}}
    }
    match &view[2][1] {
        None => {}
        Some(tile) => {if let Fish(_)=tile.content {return true;}}
    }
    false
}
fn _near_tile(view:&Vec<Vec<Option<Tile>>>,tile_type:TileType) -> bool {
    match &view[0][1] {
        None => {}
        Some(tile) => {if tile.tile_type == tile_type {return true}}
    }
    match &view[1][0] {
        None => {}
        Some(tile) => {if tile.tile_type == tile_type {return true}}
    }
    match &view[1][2] {
        None => {}
        Some(tile) => {if tile.tile_type == tile_type {return true}}
    }
    match &view[2][1] {
        None => {}
        Some(tile) => { if tile.tile_type == tile_type { return true } }
    }
    false
}
fn near_content(view:&Vec<Vec<Option<Tile>>>,content:Content) -> bool {
    match &view[0][1] {
        None => {}
        Some(tile) => {if content == tile.content {return true;}}
    }
    match &view[1][0] {
        None => {}
        Some(tile) => {if content == tile.content {return true;}}
    }
    match &view[1][2] {
        None => {}
        Some(tile) => {if content == tile.content {return true;}}
    }
    match &view[2][1] {
        None => {}
        Some(tile) => {if content == tile.content {return true;}}
    }
    false
}

