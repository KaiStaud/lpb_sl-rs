/*
State Machine controls the boot-up of the roboter.
After Powering on, Boot-Phase is entered to setup all hardware on controller side.
After leaving Boot, Cyclically Data can be transferred over the CAN-Bus.
Finally on dispatched jobs, the roboter enters the Movement-Phase in which power is supplied. 
*/

pub struct StateMachine<S> {
    pub some_unrelated_value: usize,
    pub state: S,
}

// It starts, predictably, in `Boot`
impl StateMachine<Boot> {
    pub fn new(val: String) -> Self {
        StateMachine {
            some_unrelated_value: 0,
            state: Boot::new(val)
        }
    }
}

// State Boot starts the machine with a string.
pub struct Boot {
    pub start_value: String,
}
impl Boot {
    fn new(start_value: String) -> Self {
        Boot {
            start_value: start_value,
        }
    }
}

// State Preoperational goes and breaks up that String into words.
pub struct PreOperational {
    pub interm_value: Vec<String>,
}
impl From<StateMachine<Boot>> for StateMachine<PreOperational> {
    fn from(val: StateMachine<Boot>) -> StateMachine<PreOperational> {
        StateMachine {
            some_unrelated_value: val.some_unrelated_value,
            state: PreOperational {
                interm_value: val.state.start_value.split(" ").map(|x| x.into()).collect(),
            }
        }
    }
}

// Finally, Operational gives us the length of the vector, or the word count.
pub struct Operational {
    pub final_value: usize,
}
impl From<StateMachine<PreOperational>> for StateMachine<Operational> {
    fn from(val: StateMachine<PreOperational>) -> StateMachine<Operational> {
        StateMachine {
            some_unrelated_value: val.some_unrelated_value,
            state: Operational {
                final_value: val.state.interm_value.len(),
            }
        }
    }
}