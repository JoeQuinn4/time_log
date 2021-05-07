use std::time::Duration;
use std::clone::Clone;
use stopwatch::Stopwatch;

#[derive(Clone)]
pub struct Timer {
    active_watch: Stopwatch,
}

impl Timer{
    
    pub fn is_running(&mut self) -> bool{
        self.active_watch.is_running()
    }
    pub fn start(&mut self) {
        self.active_watch.restart();
        self.active_watch.elapsed();
    }
    pub fn stop(&mut self) -> Duration{
        self.active_watch.stop();
        self.active_watch.elapsed()
    }
    pub fn get_time(&self) -> String{
        format_time(self.active_watch
            .elapsed()
            .as_secs()
        )
    }
    
}

pub fn initialize_timer() -> Timer{
    Timer{
        active_watch: Stopwatch::new(),
    }
}

#[derive(Clone)]
pub struct Record{
    projects: Vec<String>,
    times: Vec<u64>,
    size: usize,
}

impl Record {

    pub fn new() -> Record{
        Record{
            projects: Vec::new(),
            times: Vec::new(),
            size: 0,
        }
    }

    pub fn add(&mut self, p: String, d: Duration) {
        self.projects.push(p);
        self.times.push(d.as_secs());
        self.size = self.size + 1
    }
    
    pub fn get_string(&mut self, i: usize) -> String {

       // let mut s: String = "".to_string();
        //for i in 0..self.size{
        let proj: &str = &self.projects[i].clone();
        let f_time: String = format_time(self.times[i].clone());
            //s.push_str(format!("{} {}",proj,f_time).as_str());
       // }
        //s
        format!("{} {}",proj,f_time)
    }

    pub fn get_last_string(&mut self) -> String {
        self.get_string(self.size - 1)
    }

}

pub fn format_time(s: u64) -> String {
    let mut seconds: u64 = s.clone();
    let mut minutes: u64 = 0;
    let mut hours: u64 = 0;

    if seconds > 60 {
        minutes = seconds/60;
        seconds = seconds%60;
    }
    
    if minutes > 60 {
        hours = minutes/60;
        minutes = hours%60;
    }

    format!("{}:{}:{}",hours,minutes,seconds)
}

