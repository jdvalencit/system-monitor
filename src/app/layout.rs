use super::app::InputEvent;
use crate::app::app::App;
use crate::app::mainframe;
use crate::process::ProcessData;
use crate::sysinfo::Sysinfo;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io;
use std::sync::mpsc::{self};
use std::thread;
use std::time::{Duration, Instant};
use tui::widgets::TableState;
use tui::{backend::CrosstermBackend, Terminal};

pub fn tui_execute(mut sysinfo: Sysinfo) -> () {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Error while trying to create terminal.");
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let tick_rate = Duration::from_millis(1800);
    let input = {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("Error code #N") {
                    if let Ok(event_read) = event::read() {
                        if let Event::Key(key) = event_read {
                            sender.send(InputEvent::Input(key)).expect("Error code #N");
                        }
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = sender.send(InputEvent::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        receiver
    };

    let mut data: HashMap<i32, ProcessData> = HashMap::new();
    sysinfo.read_process_data(&mut data);
    let mut application = App::new(data);

    //sets the process table
    let mut table: TableState = TableState::default();
    table.select(Some(0));
    terminal
        .draw(|rect| mainframe::draw_mainframe(rect, application.borrow_mut(), table.borrow_mut()))
        .unwrap();

    loop {
        let app = application.borrow_mut();
        let table_state = table.borrow_mut();

        match input.recv() {
            Ok(InputEvent::Input(event)) => match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char('x') => {
                    break;
                }
                KeyCode::Down => {
                    if let Some(selected) = table_state.selected() {
                        if selected >= app.data().len() - 1 {
                            table_state.select(Some(0));
                        } else {
                            table_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = table_state.selected() {
                        if selected > 0 {
                            table_state.select(Some(selected - 1));
                        } else {
                            table_state.select(Some(app.data().len() - 1));
                        }
                    }
                }
                _ => {}
            },
            Ok(InputEvent::Tick) => {
                // Update data
                let mut data: HashMap<i32, ProcessData> = HashMap::new();
                sysinfo.read_process_data(&mut data);
                app.update_data(&data);
            }
            Err(_) => todo!(),
        }

        terminal
            .draw(|rect| mainframe::draw_mainframe(rect, app, table_state))
            .unwrap();
    }

    terminal.clear().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}
