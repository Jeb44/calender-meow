use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    Frame, Terminal, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{
        Block, BorderType, Padding, Paragraph, calendar::{CalendarEventStore, Monthly}
    }
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

use time::{Date, Month, OffsetDateTime};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(App::default());

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}

#[derive(Default)]
struct App {
    counter: RefCell<u8>,
}

impl App {
    fn render(&self, frame: &mut Frame) {
        let counter = self.counter.borrow();
        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a Ratzilla template.\n\
             Meow meow meow :3\n\
             Press left and right to increment and decrement the counter respectively.\n\
             Counter: {counter}",
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let horizontal = Layout::horizontal([Constraint::Percentage(50); 2]).spacing(1);
        let [top, main] = frame.area().layout(&vertical);
        let [left, right] = main.layout(&horizontal);

        let title = Line::from_iter([
            Span::from("Calendar Widget").bold(),
            Span::from(" (Press 'q' to quit)"),
        ]);
        frame.render_widget(title.centered(), top);
        frame.render_widget(paragraph, right);

        render_current_month(frame, left);
        //render_current_month(frame, right);
    }

    fn handle_events(&self, key_event: KeyEvent) {
        let mut counter = self.counter.borrow_mut();
        match key_event.code {
            KeyCode::Left => *counter = counter.saturating_sub(1),
            KeyCode::Right => *counter = counter.saturating_add(1),
            _ => {}
        }
    }
}

/// Render the current month calendar.
fn render_current_month(frame: &mut Frame, area: Rect) {
    let date = OffsetDateTime::now_utc().date();

    let monthly = Monthly::new(
        date,
        CalendarEventStore::today(Style::default().red().bold()),
    )
    .block(Block::new().padding(Padding::new(0, 0, 2, 0)))
    .show_month_header(Modifier::BOLD)
    .show_weekdays_header(Modifier::ITALIC);
    frame.render_widget(monthly, area);
}
