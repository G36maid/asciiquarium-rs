use crate::entity::EntityManager;
use crate::event::{AppEvent, Event, EventHandler};
use crate::spawning;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
};
use std::time::Instant;

/// Application with simplified architecture using death callbacks
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Entity manager for all aquarium entities
    pub entity_manager: EntityManager,
    /// Event handler
    pub events: EventHandler,
    /// Last update time for delta calculations
    pub last_update: Instant,
    /// Pause state
    pub paused: bool,
    /// Current screen bounds
    pub screen_bounds: Rect,
    /// Whether aquarium has been initialized
    pub initialized: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            entity_manager: EntityManager::new(),
            events: EventHandler::new(),
            last_update: Instant::now(),
            paused: false,
            screen_bounds: Rect::new(0, 0, 80, 24), // Default size
            initialized: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            // Get terminal size and update screen bounds
            let size = terminal.size()?;
            self.screen_bounds = Rect::new(0, 0, size.width, size.height);

            // Initialize aquarium if needed (like original's redraw)
            if !self.initialized {
                self.initialize_aquarium();
            }

            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char('p' | 'P') => self.toggle_pause(),
            KeyCode::Char('r' | 'R') => self.redraw(),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event - simplified to just update entities
    pub fn tick(&mut self) {
        if self.paused {
            return;
        }

        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update);
        self.last_update = now;

        // Simple: just update all entities
        // Death callbacks will handle all spawning automatically
        self.entity_manager
            .update_all(delta_time, self.screen_bounds);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Redraw by clearing all entities and reinitializing
    pub fn redraw(&mut self) {
        self.entity_manager = EntityManager::new();
        self.initialized = false;
    }

    /// Initialize the aquarium using the simplified spawning system
    fn initialize_aquarium(&mut self) {
        // Use the simple initialization function that matches original Perl
        spawning::initialize_aquarium(&mut self.entity_manager, self.screen_bounds);
        self.initialized = true;
    }

    /// Get entity manager reference for rendering
    pub fn entity_manager(&self) -> &EntityManager {
        &self.entity_manager
    }
}
