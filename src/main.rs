use std::{io::{self, Stdout}, borrow::Cow, path::{Path, PathBuf}};

use ropey::Rope;
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

struct Buffer {
    pub buf: Rope,
    pub file_path: Option<PathBuf>
}

impl Buffer {
    pub fn open(path: impl AsRef<Path>) -> Result<Buffer, io::Error> {
        let file = std::fs::File::open(&path)?;

        let buf = Rope::from_reader(file)?;

        return Ok(Buffer {
            buf,
            file_path: Some(path.as_ref().to_owned())
        });
    }
}

struct Window {
    buffer: usize
}

struct EditorState {
    pub buffers: Vec<Buffer>,
    pub windows: Vec<Window>
}

fn main() {
    let mut terminal = init();
    let filename = std::env::args().nth(1).expect("No argument given");

    let buffer = Buffer::open(&filename)
        .expect(&format!("Couldn't open '{filename}'"));

    terminal.draw(move |f| {
        let size = f.size();

        let title = match buffer.file_path {
            Some(path) => path.to_string_lossy().to_string(),
            None => "<scratch>".into()
        };

        let block = Block::default()
            .title(title.as_ref())
            .borders(Borders::ALL);

        let block_inner_size = block.inner(size);

        let lines = buffer.buf.lines().map(|line|
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

