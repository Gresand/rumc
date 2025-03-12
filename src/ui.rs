use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use crossterm::event::KeyCode;
use std::env;
use std::io::{self, stdout};
use crate::file_manager::list_files;
use crate::input::handle_input;

pub async fn run_ui() -> io::Result<()> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Защита от пустого $HOME
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home".to_string());

    let mut left_dir = home_dir.clone();
    let mut right_dir = home_dir.clone();
    let mut left_files = list_files(&left_dir);
    let mut right_files = list_files(&right_dir);
    
    let mut left_selected = if !left_files.is_empty() { 0 } else { usize::MAX };
    let mut right_selected = if !right_files.is_empty() { 0 } else { usize::MAX };
    
    let mut active_panel = true;

    loop {
        terminal.draw(|frame| {
            let size = frame.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            let mut left_state = ListState::default();
            let mut right_state = ListState::default();
            
            if active_panel {
                left_state.select(Some(left_selected));
            } else {
                right_state.select(Some(right_selected));
            }

            let left_list: Vec<ListItem> = left_files.iter()
                .enumerate()
                .map(|(i, f)| {
                    let style = if active_panel && i == left_selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(f.clone()).style(style)
                })
                .collect();

            let right_list: Vec<ListItem> = right_files.iter()
                .enumerate()
                .map(|(i, f)| {
                    let style = if !active_panel && i == right_selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(f.clone()).style(style)
                })
                .collect();

            let left_widget = List::new(left_list)
                .block(Block::default().title("Left Panel").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

            let right_widget = List::new(right_list)
                .block(Block::default().title("Right Panel").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

            frame.render_stateful_widget(left_widget, chunks[0], &mut left_state);
            frame.render_stateful_widget(right_widget, chunks[1], &mut right_state);
        })?;

        match handle_input().await {
            Some(KeyCode::Esc) => break,
            Some(KeyCode::Enter) => {
                if active_panel {
                    let new_path = format!("{}/{}", left_dir, left_files[left_selected]);
                    if std::path::Path::new(&new_path).is_dir() {
                        left_dir = new_path;
                        left_files = list_files(&left_dir);
                        left_selected = 0;
                    }
                } else {
                    let new_path = format!("{}/{}", right_dir, right_files[right_selected]);
                    if std::path::Path::new(&new_path).is_dir() {
                        right_dir = new_path;
                        right_files = list_files(&right_dir);
                        right_selected = 0;
                    }
                }
            }
            Some(KeyCode::Up) => {
                if active_panel {
                    if left_selected > 0 {
                        left_selected -= 1;
                    }
                } else {
                    if right_selected > 0 {
                        right_selected -= 1;
                    }
                }
            }
            Some(KeyCode::Down) => {
                if active_panel {
                    if left_selected < left_files.len().saturating_sub(1) {
                        left_selected += 1;
                    }
                } else {
                    if right_selected < right_files.len().saturating_sub(1) {
                        right_selected += 1;
                    }
                }
            }
            Some(KeyCode::Backspace) => {
                if active_panel {
                    if left_dir != "/" {
                        if let Some(parent) = std::path::Path::new(&left_dir).parent() {
                            let parent_str = parent.to_string_lossy().to_string();
                            if parent_str >= home_dir || parent_str == "/" {
                                left_dir = parent_str;
                                left_files = list_files(&left_dir);
                                left_selected = 0;
                            }
                        }
                    }
                } else {
                    if right_dir != "/" {
                        if let Some(parent) = std::path::Path::new(&right_dir).parent() {
                            let parent_str = parent.to_string_lossy().to_string();
                            if parent_str >= home_dir || parent_str == "/" {
                                right_dir = parent_str;
                                right_files = list_files(&right_dir);
                                right_selected = 0;
                            }
                        }
                    }
                }
            }
            Some(KeyCode::Tab) => {
                active_panel = !active_panel; // Переключение между панелями
            }
            _ => {}
        }
    }
    Ok(())
}

