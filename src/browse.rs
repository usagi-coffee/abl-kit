use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::error::Error;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::stdout;

struct Field {
    name: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut table = String::new();
    let fields: Vec<Field> = vec![];

    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(3),
                    Constraint::Percentage(15),
                    Constraint::Percentage(80),
                ])
                .split(frame.size());

            frame.render_widget(
                Paragraph::new("Browse (press 'F4' to quit)")
                    .white()
                    .alignment(Alignment::Center),
                layout[0],
            );

            // Query
            let query_block = Block::new().title("Query").borders(Borders::ALL);
            let query_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    fields
                        .iter()
                        .map(|_| Constraint::Percentage(10))
                        .chain(std::iter::once(Constraint::Percentage(10)))
                        .collect::<Vec<Constraint>>(),
                )
                .split(layout[1].inner(&Margin {
                    horizontal: 1,
                    vertical: 1,
                }));

            let table_input = Paragraph::new(table.clone())
                .block(Block::new().title("Table").borders(Borders::ALL));

            frame.render_widget(query_block, layout[1]);
            frame.render_widget(table_input, query_layout[0]);

            for (i, field) in fields.iter().enumerate() {
                let field_input = Paragraph::new(table.clone())
                    .block(Block::new().title(field.name.clone()).borders(Borders::ALL));

                frame.render_widget(field_input, query_layout[1 + i]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::F(4) => break,
                    KeyCode::Backspace => {
                        table.pop();
                    }
                    KeyCode::Char(character) => {
                        table.push(character);
                    }
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
