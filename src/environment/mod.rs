use std::sync::mpsc::Receiver;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;

pub fn start_env(reciever:Receiver<i32>){
    println!("{}",reciever.recv().unwrap());
}

struct MyRobot{

}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        todo!()
    }

    fn handle_event(&mut self, event: Event) {
        todo!()
    }

    fn get_energy(&self) -> &Energy {
        todo!()
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        todo!()
    }

    fn get_coordinate(&self) -> &Coordinate {
        todo!()
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        todo!()
    }

    fn get_backpack(&self) -> &BackPack {
        todo!()
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        todo!()
    }
}