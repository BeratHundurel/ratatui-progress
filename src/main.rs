use std::{io, sync::mpsc, thread};

use ratatui::{
    buffer::Buffer,
    crossterm::{
        self,
        event::{KeyCode, KeyEventKind},
    },
    layout::{Constraint, Layout, Rect},
    style::{self, Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Gauge, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App {
        exit: false,
        progress_bar_color: Color::Green,
        background_progress: 0_f64,
    };

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let tx_to_background_progress_events = event_tx.clone();
    thread::spawn(move || {
        run_background_thread(tx_to_background_progress_events);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

enum Event {
    Input(crossterm::event::KeyEvent),
    Progress(f64),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read() {
            Ok(crossterm::event::Event::Key(key_event)) => {
                tx.send(Event::Input(key_event)).unwrap();
            }
            _ => {}
        }
    }
}

fn run_background_thread(tx: mpsc::Sender<Event>) {
    std::thread::spawn(move || {
        for i in 0..=100 {
            tx.send(Event::Progress(i as f64 / 100.0)).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

pub struct App {
    exit: bool,
    progress_bar_color: Color,
    background_progress: f64,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Progress(progress) => self.background_progress = progress,
            }

            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('c') {
            self.progress_bar_color = match self.progress_bar_color {
                Color::Green => Color::Red,
                Color::Red => Color::Blue,
                Color::Blue => Color::Green,
                _ => Color::Green,
            };
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical_layout =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);

        Line::from("Process overview")
            .bold()
            .render(title_area, buf);

        let instructions = Line::from(vec![
            " Change color ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])
        .centered();

        let block = Block::bordered()
            .title(Line::from(" Background processes "))
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .label(format!(
                "Process 1: {:.2}%",
                self.background_progress * 100_f64
            ))
            .ratio(self.background_progress);

        progress_bar.render(
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top(),
                width: gauge_area.width,
                height: 3,
            },
            buf,
        );
    }
}
