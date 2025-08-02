use crate::app::App;

pub mod app;
pub mod depth;
pub mod entities;
pub mod entity;
pub mod event;
pub mod spawning;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
