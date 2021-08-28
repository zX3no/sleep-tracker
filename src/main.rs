#![allow(unused_imports)]
use crossterm::execute;
use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Widget};
use tui::{symbols, Terminal};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    enable_raw_mode().unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let datasets = vec![
        Dataset::default()
            .name("August")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&[
                (1.0, 8.0),
                (2.0, 9.0),
                (3.0, 9.0),
                (4.0, 7.0),
                (5.0, 12.0),
                (6.0, 8.0),
                (7.0, 9.0),
                (8.0, 10.0),
                (9.0, 7.0),
                (10.0, 8.0),
                (11.0, 6.0),
                (12.0, 8.0),
                (13.0, 12.0),
                (14.0, 9.0),
                (15.0, 7.0),
                (16.0, 6.0),
                (17.0, 9.0),
                (18.0, 13.0),
                (19.0, 10.0),
                (20.0, 5.0),
                (21.0, 9.0),
                (22.0, 7.0),
                (23.0, 9.0),
            ]),
        Dataset::default()
            .name("July")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&[
                (1.0, 9.0),
                (2.0, 7.0),
                (3.0, 8.0),
                (4.0, 9.0),
                (5.0, 11.0),
                (6.0, 8.0),
                (7.0, 2.0),
                (8.0, 10.0),
                (9.0, 7.0),
                (10.0, 6.0),
                (11.0, 6.0),
                (12.0, 5.0),
                (13.0, 10.0),
                (14.0, 8.0),
                (15.0, 9.0),
                (16.0, 7.0),
                (17.0, 8.0),
                (18.0, 9.0),
                (19.0, 4.0),
                (20.0, 6.0),
                (21.0, 8.0),
                (22.0, 9.0),
                (23.0, 7.0),
            ]),
    ];

    terminal.clear()?;

    terminal.draw(|f| {
        let area = f.size();
        let chart = Chart::new(datasets)
            .block(Block::default().title("Chart"))
            .x_axis(
                Axis::default()
                    .title(Span::styled("Date", Style::default().fg(Color::Cyan)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 30.0])
                    .labels(
                        ["0", "10", "20", "30"]
                            .iter()
                            .cloned()
                            .map(Span::from)
                            .collect(),
                    ),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled(
                        " Time Slept",
                        Style::default().fg(Color::Cyan),
                    ))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 16.0])
                    .labels(
                        ["0", "4", "8", "12", "16"]
                            .iter()
                            .cloned()
                            .map(Span::from)
                            .collect(),
                    ),
            );

        f.render_widget(chart, area);
    })?;

    Ok(())
}
