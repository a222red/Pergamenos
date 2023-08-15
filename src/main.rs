use std::{io::Stdout, borrow::Cow};

use ropey::{Rope, RopeSlice, RopeBuilder};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph}, text::{Span, Line}, style::Style, prelude::Alignment
};
use crossterm::{
    event::{self, Event, KeyEvent, KeyModifiers, KeyEventKind, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

fn init() -> Terminal {
    enable_raw_mode().expect("Unable to enable raw mode");

    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen)
        .expect("Couldn't enter alternate screen");

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).expect("Unable to init TUI");

    return terminal;
}

fn deinit(terminal: &mut Terminal) {
    disable_raw_mode().expect("Couldn't disable raw mode");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .expect("Couldn't leave alternate screen");
}

fn main() {
    let mut terminal = init();
    let filename = std::env::args().nth(1).expect("No argument given");

    let file = std::fs::File::open(&filename)
        .expect("Couln't open '{filename}'");

    let buffer = Rope::from_reader(file)
        .expect("Couln't read from '{filename}'");

    terminal.draw(move |f| {
        let size = f.size();

        let block = Block::default()
            .title(filename)
            .borders(Borders::ALL);

        let block_inner_size = block.inner(size);

        let lines = buffer.lines().map(|line|
            Line {
                spans: line.chunks().map(|slice|
                    Span {
                        content: Cow::Borrowed(slice),
                        style: Style::default()
                    }
                ).collect::<Vec<_>>(),
                alignment: Some(Alignment::Left)
            }
        ).collect::<Vec<_>>();

        let text = Paragraph::new(lines);

        f.render_widget(block, size);
        f.render_widget(text, block_inner_size);
    }).expect("Couldn't draw UI");

    loop {
        let ev = event::read().expect("Couldn't get event");

        match ev {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _
            }) => break,
            _ => ()
        }
    }

    deinit(&mut terminal);
}

