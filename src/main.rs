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

    let datasets = vec![Dataset::default()
        .name("data1")
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Scatter)
        .style(Style::default().fg(Color::Magenta))
        .data(&[(1.0, 8.0), (2.0, 9.0), (3.0, 9.0), (4.0, 7.0), (5.0, 8.0)])];

    terminal.clear()?;

    terminal.draw(|f| {
        let area = Rect::new(0, 0, 100, 40);
        let chart = Chart::new(datasets)
            .block(Block::default().title("Chart"))
            .x_axis(
                Axis::default()
                    .title(Span::styled("Date", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 10.0])
                    .labels(
                        ["0.0", "5.0", "10.0"]
                            .iter()
                            .cloned()
                            .map(Span::from)
                            .collect(),
                    ),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled(" Time Slept", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 14.0])
                    .labels(
                        ["0.0", "7.0", "14.0"]
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
