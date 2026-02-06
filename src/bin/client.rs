use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    event::{self, Event as CEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;
use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    net::TcpStream,
    sync::mpsc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

const ADDRESS: &str = "127.0.0.1:7878";

enum Event {
    Input(crossterm::event::KeyEvent),
    Redraw,
}

fn handle_received_messages(
    mut read_stream: TcpStream,
    messages: Arc<Mutex<VecDeque<String>>>,
    notify: mpsc::Sender<Event>,
) {
    loop {
        let mut buffer = [0; 256];
        match read_stream.read(&mut buffer) {
            Ok(0) => break, // Connection has been closed by the server
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
                let msg = msg.trim_end_matches('\n').to_string();
                let mut messages = messages.lock().unwrap();
                if messages.len() == 10 {
                    messages.pop_front();
                }
                messages.push_back(msg);
                let _ = notify.send(Event::Redraw);
            }
            Err(e) => {
                eprintln!("Failed to receive message: {:?}", e);
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(), Clear(ClearType::All))?;
    let mut stream = TcpStream::connect(ADDRESS)?;

    let messages = Arc::new(Mutex::new(VecDeque::new()));
    let (tx, rx) = mpsc::channel::<Event>();

    // Socket receive thread
    let read_stream = stream.try_clone()?;
    let messages_clone = Arc::clone(&messages);
    let tx_recv = tx.clone();
    thread::spawn(move || handle_received_messages(read_stream, messages_clone, tx_recv));

    // Terminal initialization
    let term_stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(term_stdout);
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;

    // Key input thread
    let tx_input = tx.clone();
    thread::spawn(move || loop {
        match event::poll(Duration::from_millis(100)) {
            Ok(true) => match event::read() {
                Ok(CEvent::Key(key)) => {
                    let _ = tx_input.send(Event::Input(key));
                }
                Ok(_) => {}
                Err(_) => break,
            },
            Ok(false) => {}
            Err(_) => break,
        }
    });

    let mut input = String::new();
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
                .split(f.size());

            let messages = messages
                .lock()
                .unwrap()
                .iter()
                .map(|i| i.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            let message_widget = Paragraph::new(messages)
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(message_widget, chunks[0]);

            let input_widget = Paragraph::new(input.as_ref())
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input_widget, chunks[1]);
        })?;

        match rx.recv_timeout(Duration::from_millis(250)) {
            Ok(Event::Input(key)) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    should_quit = true;
                }
                crossterm::event::KeyCode::Char(c) => {
                    if c == 'c'
                        && key
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL)
                    {
                        should_quit = true;
                    } else if c == 'q' {
                        should_quit = true;
                    } else {
                        input.push(c);
                    }
                }
                crossterm::event::KeyCode::Backspace => {
                    input.pop();
                }
                crossterm::event::KeyCode::Enter => {
                    if let Err(e) = stream.write_all(input.as_bytes()) {
                        eprintln!("Failed to send message: {:?}", e);
                    }
                    if let Err(e) = stream.flush() {
                        eprintln!("Failed to flush message: {:?}", e);
                    }
                    input.clear();
                }
                _ => {}
            },
            Ok(Event::Redraw) => {}
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), Clear(ClearType::All));
    Ok(())
}

