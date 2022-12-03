use crate::app::app::App;
use crate::process::ProcessData;
use std::cmp::Ordering::Equal;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState};
use tui::Frame;

pub fn draw_mainframe<B>(frame: &mut Frame<B>, app: &mut App, table_state: &mut TableState)
where
    B: Backend,
{
    let mut size = frame.size();
    if size.width > 138 && size.width < 165 {
        size.width = 138;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(0), Constraint::Min(0)].as_ref())
        .split(size);

    let data = app.data();
    let process = {
        let primary = Style::default().fg(Color::LightBlue);
        let secondary = Style::default().fg(Color::LightGreen);
        let title = Style::default().fg(Color::Gray);

        let mut rows = vec![];
        let mut proc_list : Vec<ProcessData> = Vec::from_iter(data.values().cloned());

        proc_list.sort_by(|a, b| {
            if b.cpu_usage_percent == a.cpu_usage_percent {
                b.mem_usage_percent
                    .partial_cmp(&a.mem_usage_percent)
                    .unwrap_or(Equal)
            } else {
                b.cpu_usage_percent
                    .partial_cmp(&a.cpu_usage_percent)
                    .unwrap_or(Equal)
            }
        });

        for process in proc_list {
            let disk_rbytes = process.disk_read_bytes.unwrap_or(0) / 1000;
            let disk_wbytes = process.disk_write_bytes.unwrap_or(0) / 1000;

            let row = Row::new(vec![
                Cell::from(Span::styled(process.pid.to_string(), primary)),
                Cell::from(Span::styled(process.parent_pid.to_string(), secondary)),
                Cell::from(Span::styled(process.priority.to_string(), primary)),
                Cell::from(Span::styled(process.name.to_string(), secondary)),
                Cell::from(Span::styled(
                    format!("{:.5}", process.cpu_usage_percent.to_string()),
                    primary,
                )),
                Cell::from(Span::styled(
                    format!("{:.5}", process.mem_usage_percent.to_string()),
                    secondary,
                )),
                Cell::from(Span::styled(
                    format!("{:.5}", disk_rbytes.to_string()),
                    secondary,
                )),
                Cell::from(Span::styled(
                    format!("{:.5}", disk_wbytes.to_string()),
                    primary,
                )),
                Cell::from(Span::styled(process.status.clone(), secondary)),
                Cell::from(Span::styled(process.uid.unwrap().to_string(), primary)),
                Cell::from(Span::styled(process.command.to_string(), secondary)),
            ]);
            rows.push(row);
        }

        Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title("Process Info"),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .widths(&[
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(1100),
            ])
            .column_spacing(3)
            .header(
                Row::new(vec![
                    "PID",
                    "PPID",
                    "PRIO",
                    "NAME",
                    "CPU%",
                    "MEM%",
                    "READ(KB)",
                    "WRITE(KB)",
                    "STATUS",
                    "USER",
                    "COMMAND",
                ])
                .style(title)
                .bottom_margin(1),
            )
    };

    frame.render_stateful_widget(process, chunks[1], table_state);
}
