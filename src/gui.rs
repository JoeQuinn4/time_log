
use druid::{widget::{Align, Button, Flex, Label, TextBox}};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};
//use std::time::Duration;

use crate::time_log_core::{self, initialize_timer};
use crate::time_log_core::Timer;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<TimeLogState> = LocalizedString::new("time_log");
const WINDOW_WIDTH: f64 = 400.0;
const WINDOW_HEIGHT: f64 = 500.0;


#[derive(Clone, Data, Lens)]
struct TimeLogState{
    name: String,
    #[data(ignore)]
    timer: Timer,
}

pub fn start_gui() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT));

    let timer = initialize_timer();
    // create the initial app state
    let initial_state = TimeLogState {
        name: String::from("time_log init"),
        timer: timer,
    };

    // start the application
    AppLauncher::with_window(main_window)
        /* .configure_env(|env, _state| {
            env.set(TIMER_KEY, &timer);
        })*/
        .launch(initial_state)
        .expect("Failed to launch application");
        
}

fn build_root_widget<'a>() -> impl Widget<TimeLogState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &TimeLogState, _env: &Env| format!("{}", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .fix_width(TEXT_BOX_WIDTH)
        .lens(TimeLogState::name);
    
    let button = Button::new("start/stop")
        .on_click(|_event, _data: &mut TimeLogState, _env| {
            toggle(&mut _data.timer);
        });

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Flex::row()
                        .with_child(textbox)
                        .with_child(button)
                    );

    // center the two widgets in the available space
    Align::centered(layout)
}

fn toggle(timer: &mut Timer){
    let duration = time_log_core::timer_toggle(timer);
    let seconds = duration.as_secs();
    let frac_seconds = duration.as_millis();
    if seconds==0 && frac_seconds==0{
        println!("clock started")
    } else {
        println!("clock stopped: {}",seconds);
    }
}