use std::time::Duration;
use std::clone::Clone;
use stopwatch::Stopwatch;

#[derive(Clone)]
pub struct Timer {
    active_watch: Stopwatch,
}

impl Timer{
    
    //pub fn try_from_value(v: &'a Value) -> Result<Self, ValueTypeError>{}
    fn is_running(&mut self) -> bool{
        self.active_watch.is_running()
    }
    fn start(&mut self) -> Duration{
        self.active_watch.restart();
        self.active_watch.elapsed()
    }
    fn stop(&mut self) -> Duration{
        self.active_watch.stop();
        self.active_watch.elapsed()
    }
    
}

pub fn initialize_timer() -> Timer{
    Timer{
        active_watch: Stopwatch::new(),
    }
}

pub fn timer_toggle(timer: &mut Timer) -> Duration{
    if timer.is_running() {
        timer.stop()
    } else {
        timer.start()
    }
}

