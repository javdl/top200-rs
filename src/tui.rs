use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal, Frame,
};
use std::io;

pub struct App {
    items: Vec<String>,
    selected: Option<usize>,
}

impl App {
    pub fn new(items: Vec<String>) -> App {
        App {
            items,
            selected: None,
        }
    }

    pub fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<Option<String>> {
    loop {
        terminal.draw(|f| {
            draw_ui(f, &app);
            ()
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(None),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => {
                    return Ok(app.selected.map(|i| app.items[i].clone()));
                }
                _ => {}
            }
        }
    }
}

fn draw_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    let title = Paragraph::new("Stock Market Analysis")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if Some(i) == app.selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![Span::styled(item, style)]))
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(items, chunks[1]);
}

pub fn start_tui(options: Vec<String>) -> Result<Option<String>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(options);
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}
