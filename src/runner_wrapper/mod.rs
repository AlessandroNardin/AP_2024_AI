use std::sync::mpsc::Receiver;
use robotics_lib::runner::Runner;

pub(crate) mod robot;

struct RunnerWrapper{
    runner: Runner,
    control_receiver:Receiver<u8>,
}
