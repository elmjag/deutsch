use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Constraint, Layout};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    widgets::Widget,
    DefaultTerminal, Frame,
};
use std::io;

mod deck;
use deck::Deck;

#[derive(Debug)]
struct App {
    exit: bool,
    deck: Deck,
    answer: Answer,
}

#[derive(Debug)]
struct Answer {
    text: String,
    wrong: bool,
}

impl Answer {
    fn new() -> Answer {
        Answer {
            text: String::from(""),
            wrong: false,
        }
    }

    fn rendered(&self) -> String {
        let mut res = String::from("___");
        let text_len = self.text.len();

        res.replace_range(0..text_len, &self.text);

        res
    }

    fn add_character(&mut self, c: char) {
        if self.text.len() >= 3 {
            return;
        }
        self.text.push(c);
    }

    fn delete_character(&mut self) {
        self.text.pop();
    }

    fn reset(&mut self) {
        self.text.clear();
        self.wrong = false;
    }
}

impl App {
    fn new() -> App {
        App {
            exit: false,
            answer: Answer::new(),
            deck: Deck::new(),
        }
    }

    fn check_answer(&mut self) {
        let noun = self.deck.get_current_noun();
        if self.answer.text != noun.article {
            /* incorrect answer */
            self.answer.wrong = true;
            return;
        }

        /* the answer was correct */
        self.answer.reset();
        self.exit = self.deck.goto_next_noun().is_err();
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.answer.wrong {
            self.answer.reset();
        }

        if key_event.kind != KeyEventKind::Press {
            return;
        }

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.exit = true,
            KeyCode::Enter => self.check_answer(),
            KeyCode::Backspace => self.answer.delete_character(),
            KeyCode::Char(c) => self.answer.add_character(c),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        };

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create a vertical layout with centered content
        let layout = Layout::vertical([
            Constraint::Percentage(50), // Top space
            Constraint::Length(1),      // Centered line
            Constraint::Percentage(50), // Bottom space
        ])
        .split(area);

        let guess_background = match self.answer.wrong {
            false => Color::Reset,
            true => Color::Red,
        };

        let noun = self.deck.get_current_noun();

        let line = Line::from(vec![
            Span::styled(self.answer.rendered(), Style::new().bg(guess_background)),
            Span::raw(" "),
            Span::styled(noun.word, Style::new()),
        ]);

        Paragraph::new(line).centered().render(layout[1], buf);
    }
}

fn main() -> io::Result<()> {
    let mut app = App::new();
    let mut terminal = ratatui::init();

    let res = app.run(&mut terminal);

    ratatui::restore();

    println!("bis bald");
    res
}
