use log::debug;

use crate::vector3::Vector3;

pub enum State {
    Initialization,
    Sleep(SleepState),
    Standing(StandingState),
    Walking(WalkingState),
}

pub struct SleepState {
    target: Vector3,
    target_reached: bool,
}

pub struct StandingState {
    points: i32,
}

pub struct WalkingState {
    points: i32,
}

impl State {
    pub fn init(&mut self) {
        match self {
            State::Initialization => initialization(),
            State::Sleep(state) => init_sleep(state),
            State::Standing(state) => init_standing(state),
            State::Walking(state) => init_walking(state),
        }
    }

    pub fn run_loop(&mut self) {
        match self {
            State::Initialization => (),
            State::Sleep(state) => loop_sleep(state),
            State::Standing(state) => loop_standing(state),
            State::Walking(state) => loop_walking(state),
        }
    }
}

fn initialization() {
    debug!("Initialization State.");
    for _i in 0..6 {
        // TODO define method and static data
        //moveToPos(i, baseLegConfigurationPosition);
    }
    //Systick::delay(1000.millis()).await;
}

fn init_sleep(_state: &mut SleepState) {
    todo!()
}

fn loop_sleep(_state: &mut SleepState) {
    todo!()
}

fn init_standing(_state: &mut StandingState) {
    todo!()
}

fn loop_standing(_state: &mut StandingState) {
    todo!()
}

fn init_walking(_state: &mut WalkingState) {
    todo!()
}

fn loop_walking(_state: &mut WalkingState) {
    todo!()
}
