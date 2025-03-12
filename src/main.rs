mod ui;
mod file_manager;
mod vfs;
mod input;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};
use ui::run_ui;
use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    // Создаём асинхронный рантайм вручную
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _ = rt.block_on(run_ui()); // Запускаем UI в асинхронном режиме

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
