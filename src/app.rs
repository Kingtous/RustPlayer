pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Routes {
    Main,
    Help,
}

#[derive(PartialEq)]
pub enum ActiveModules {
    Fs,
    MusicController,
}

use crate::{
    config::Config,
    fs::FsExplorer,
    handler::handle_keyboard_event,
    main,
    media::player::{MusicPlayer, Player},
    ui::{
        fs::draw_fs_tree,
        help::draw_help,
        music_board::{draw_music_board, MusicController},
        EventType,
    },
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use failure::Error;
use std::{
    io::{stdout, Stdout},
    sync::mpsc,
    thread::{self, sleep_ms},
    time::{self, Duration, SystemTime},
    vec,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Widget, Wrap},
    Frame, Terminal,
};

pub struct App {
    pub mode: InputMode,
    pub fs: FsExplorer,
    pub route_stack: Vec<Routes>,
    pub player: MusicPlayer,
    pub music_controller: MusicController,
    pub active_modules: ActiveModules,
    pub config: Config,
    // terminal: Option<Terminal<B>>,
    msg: String,
}

impl App {
    pub fn new() -> Option<Self> {
        Some(Self {
            mode: InputMode::Normal,
            fs: FsExplorer::default(Some(|err| {
                eprintln!("{}", err);
            }))
            .ok()?,
            // terminal: None,
            route_stack: vec![Routes::Main],
            player: Player::new(),
            music_controller: MusicController {
                state: ListState::default(),
            },
            active_modules: ActiveModules::Fs,
            msg: "Welcome to RustPlayer".to_string(),
            config: Config::default(),
        })
    }

    // block thread and show screen
    pub fn run(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        execute!(terminal.backend_mut(), DisableMouseCapture);
        enable_raw_mode()?;
        terminal.hide_cursor()?;
        self.draw_frame(&mut terminal)?;
        // tick daemon thread
        let (sd, rd) = mpsc::channel::<EventType>();
        let tick = self.config.tick_gap.clone();
        thread::spawn(move || loop {
            thread::sleep(tick);
            sd.send(EventType::Player);
        });
        // start event
        loop {
            if event::poll(self.config.refresh_rate)? {
                if let Event::Key(key) = event::read()? {
                    match self.mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('q') => {
                                break;
                            }
                            code => {
                                handle_keyboard_event(self, code);
                            }
                        },
                        InputMode::Editing => {}
                    }
                } else {
                    thread::sleep(self.config.refresh_rate);
                }
            } else {
                thread::sleep(self.config.refresh_rate);
            }
            // 10 fps
            self.draw_frame(&mut terminal)?;
            if let Ok(event) = rd.try_recv() {
                self.handle_events(event);
            }
        }
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn handle_events(&mut self, event: EventType) {
        // event
        match event {
            EventType::Player => {
                let player = &mut self.player;
                player.tick();
            }
        }
    }

    pub fn draw_frame<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Error>
    where
        B: Backend,
    {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
                .split(size);
            if self.route_stack.is_empty() {
                self.route_stack.push(Routes::Main);
            }
            let route = self.route_stack.last().unwrap();
            match route {
                Routes::Main => {
                    self.draw_header(frame, chunks[0]);
                    self.draw_body(frame, chunks[1]);
                }
                Routes::Help => {
                    self.draw_header(frame, chunks[0]);
                    draw_help(self,frame, chunks[1]);
                }
            }
        })?;
        Ok(())
    }

    pub fn draw_header<B>(&mut self, frame: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let block = Block::default()
            .title("RustPlayer - Music Player For Rust")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White));
        let msg_p = Paragraph::new(Text::from(self.msg.as_str()))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(block)
            .wrap(Wrap { trim: true });
        // total
        frame.render_widget(msg_p, area);
    }

    pub fn draw_body<B>(&mut self, frame: &mut Frame<B>, area: Rect) -> Result<(), Error>
    where
        B: Backend,
    {
        let route = self.route_stack.first().unwrap();
        match route {
            Routes::Main => {
                let main_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    // .margin(2)
                    .split(area);
                // 左侧
                draw_fs_tree(self, frame, main_layout[0]);
                // 右侧
                draw_music_board(self, frame, main_layout[1]);
            }
            Routes::Help => {
                draw_help(self, frame, area);
            }
        }
        Ok(())
    }
}
