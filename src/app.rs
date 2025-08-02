use crate::entities::{Bubble, Fish};
use crate::entity::EntityManager;
use crate::event::{AppEvent, Event, EventHandler};
use rand::Rng;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
};
use std::time::{Duration, Instant};

/// Application.

pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Entity manager for all aquarium entities
    pub entity_manager: EntityManager,
    /// Event handler.
    pub events: EventHandler,
    /// Last update time for delta calculations
    pub last_update: Instant,
    /// Pause state
    pub paused: bool,
    /// Time since last fish spawn
    pub last_fish_spawn: Instant,
    /// Time since last bubble spawn
    pub last_bubble_spawn: Instant,
}

impl Default for App {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            running: true,
            entity_manager: EntityManager::new(),
            events: EventHandler::new(),
            last_update: now,
            paused: false,
            last_fish_spawn: now,
            last_bubble_spawn: now,
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
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        if self.paused {
            return;
        }

        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update);
        self.last_update = now;

        // Screen bounds will be updated in render, use last known size or default
        // This is a limitation - we'll improve this when we refactor the update cycle
        let screen_bounds = Rect::new(0, 0, 80, 24); // Default size for now

        // Update all entities
        self.entity_manager.update_all(delta_time, screen_bounds);

        // Spawn new fish periodically
        self.maybe_spawn_fish(screen_bounds);

        // Spawn bubbles from random fish
        self.maybe_spawn_bubbles();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Redraw by clearing all entities and respawning
    pub fn redraw(&mut self) {
        self.entity_manager = EntityManager::new();
        self.last_fish_spawn = Instant::now();
    }

    /// Maybe spawn a new fish based on population and timing
    fn maybe_spawn_fish(&mut self, screen_bounds: Rect) {
        let now = Instant::now();

        // Calculate target fish population based on screen size (original formula)
        let screen_size =
            (screen_bounds.height.saturating_sub(9)) as usize * screen_bounds.width as usize;
        let target_fish_count = screen_size / 350;

        let current_fish_count = self.entity_manager.get_entities_by_type("fish").len();

        // Spawn fish if below target and enough time has passed
        if current_fish_count < target_fish_count
            && now.duration_since(self.last_fish_spawn) > Duration::from_millis(500)
        {
            let fish_id = self.entity_manager.get_next_id();
            let fish = Fish::new_random(fish_id, screen_bounds);
            self.entity_manager.add_entity(Box::new(fish));
            self.last_fish_spawn = now;
        }
    }

    /// Maybe spawn bubbles from random fish
    fn maybe_spawn_bubbles(&mut self) {
        let now = Instant::now();

        // Only spawn bubbles every 2-4 seconds
        if now.duration_since(self.last_bubble_spawn) < Duration::from_millis(2000) {
            return;
        }

        let fish_entities = self.entity_manager.get_entities_by_type("fish");
        if fish_entities.is_empty() {
            return;
        }

        // Random chance to spawn a bubble (about 30% chance when timer allows)
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) {
            // Pick a random fish to emit a bubble
            let fish_index = rng.gen_range(0..fish_entities.len());
            let fish = fish_entities[fish_index];

            // Create bubble at fish position
            let fish_pos = fish.position();
            let sprite = fish.get_current_sprite();
            let (width, _) = sprite.get_bounding_box();

            // Determine fish direction based on velocity or position heuristic
            let fish_moving_right = fish_pos.x < 40.0; // Simple heuristic based on screen position

            let bubble_id = self.entity_manager.get_next_id();
            let bubble = Bubble::from_fish_position(bubble_id, fish_pos, width, fish_moving_right);
            self.entity_manager.add_entity(Box::new(bubble));

            self.last_bubble_spawn = now;
        }
    }

    /// Get entity manager reference for rendering
    pub fn entity_manager(&self) -> &EntityManager {
        &self.entity_manager
    }

    /// Update method that can be called from render with actual screen bounds
    pub fn update_with_bounds(&mut self, screen_bounds: Rect) {
        if self.paused {
            return;
        }

        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update);
        self.last_update = now;

        // Update all entities with actual screen bounds
        self.entity_manager.update_all(delta_time, screen_bounds);

        // Spawn new fish periodically
        self.maybe_spawn_fish(screen_bounds);
    }
}
