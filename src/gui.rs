
use druid::{widget::{Align, Button, Flex, Label, TextBox}};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

use crate::time_log_core::{initialize_timer};
use crate::time_log_core::{Timer,Record};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<TimeLogState> = LocalizedString::new("time_log");
const WINDOW_WIDTH: f64 = 400.0;
const WINDOW_HEIGHT: f64 = 500.0;
const LOG_HEIGHT: f64 = WINDOW_HEIGHT * 0.60;


#[derive(Clone, Data, Lens)]
struct TimeLogState{
    live_text: String,
    current_proj: String,
    #[data(ignore)]
    timer: Timer,
    #[data(ignore)]
    record: Record,
    log: String,
}

pub fn start_gui() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT));

    // create the initial app state
    let initial_state = TimeLogState {
        live_text: String::from("time_log init"),
        current_proj: String::new(),
        timer: initialize_timer(),
        record: Record::new(),
        log: String::new(),
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
    let active_label = Label::new(
        |data: &TimeLogState, _env: &Env| 
            format!("{} - {}", data.current_proj, data.timer.get_time())
    );

    let logged_label = Label::new(
        |data: &TimeLogState, _env:&Env|
            format!("{}",data.log)
    );

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .fix_width(TEXT_BOX_WIDTH)
        .lens(TimeLogState::live_text);
    
    let start_button = Button::new("start")
        .on_click(|_event, _data: &mut TimeLogState, _env| {
            start(_data);
        });
    let stop_button = Button::new("stop")
        .on_click(|_event, _data: &mut TimeLogState, _env|{
            stop(_data);
        });

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(active_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Flex::row()
                        .with_child(textbox)
                        .with_child(Flex::column()
                            .with_child(start_button)
                            .with_child(stop_button)
                        )
                )
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(logged_label.fix_height(LOG_HEIGHT));

    // center the two widgets in the available space
    Align::centered(layout)
}

fn start(mut data: &mut TimeLogState){
    if data.timer.is_running() {
        data = stop(data);
    }
    data.current_proj=data.live_text.clone();
    data.timer.start();
}

fn stop(data: &mut TimeLogState) -> &mut TimeLogState{

    if data.timer.is_running(){
            
        let duration = data.timer.stop();
        let seconds = duration.as_secs();
        println!("clock stopped: {}",seconds);

        data.record.add(data.current_proj.clone(), duration);
        data.log = format!("{}\n{}",data.record.get_last_string(),data.log);
        data.current_proj = "".to_string();
    }
    data
}


