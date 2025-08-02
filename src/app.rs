use crate::entities::{
    Bubble, Castle, CastleManager, Fish, Seaweed, SeaweedManager, WaterSurfaceManager,
};
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
    /// Current screen bounds
    pub screen_bounds: Rect,
    /// Water surface manager
    pub water_surface_manager: WaterSurfaceManager,
    /// Whether water surface has been initialized
    pub water_initialized: bool,
    /// Seaweed manager
    pub seaweed_manager: SeaweedManager,
    /// Castle manager
    pub castle_manager: CastleManager,
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
            screen_bounds: Rect::new(0, 0, 80, 24), // Default size
            water_surface_manager: WaterSurfaceManager::new(),
            water_initialized: false,
            seaweed_manager: SeaweedManager::new(),
            castle_manager: CastleManager::new(),
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

        // Use stored screen bounds
        let screen_bounds = self.screen_bounds;

        // Initialize water surface if not done yet
        self.maybe_initialize_water_surface();

        // Update water surface for screen size changes
        self.update_water_surface();

        // Update all entities
        self.entity_manager.update_all(delta_time, screen_bounds);

        // Spawn new fish periodically
        self.maybe_spawn_fish(screen_bounds);

        // Spawn bubbles from random fish
        self.maybe_spawn_bubbles();

        // Manage seaweed population
        self.maybe_spawn_seaweed(screen_bounds);

        // Manage castle
        self.maybe_spawn_castle(screen_bounds);
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
        self.water_initialized = false; // Force water surface recreation
        self.seaweed_manager = SeaweedManager::new(); // Reset seaweed manager
        self.castle_manager = CastleManager::new(); // Reset castle manager
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
            && now.duration_since(self.last_fish_spawn) > Duration::from_millis(2000)
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

        // Only spawn bubbles every 0.5-1 seconds for testing
        if now.duration_since(self.last_bubble_spawn) < Duration::from_millis(500) {
            return;
        }

        let fish_entities = self.entity_manager.get_entities_by_type("fish");
        if fish_entities.is_empty() {
            return;
        }

        // Random chance to spawn a bubble (about 80% chance when timer allows for testing)
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.8) {
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

    /// Maybe spawn seaweed based on population and timing
    fn maybe_spawn_seaweed(&mut self, screen_bounds: Rect) {
        // Update target count based on screen size
        self.seaweed_manager.update_target_count(screen_bounds);

        let current_seaweed_count = self.entity_manager.get_entities_by_type("seaweed").len();

        if self
            .seaweed_manager
            .should_spawn_seaweed(current_seaweed_count)
        {
            let seaweed_id = self.entity_manager.get_next_id();
            let seaweed = Seaweed::new_random(seaweed_id, screen_bounds);
            self.entity_manager.add_entity(Box::new(seaweed));
        }
    }

    /// Maybe spawn castle based on screen size and manager state
    fn maybe_spawn_castle(&mut self, screen_bounds: Rect) {
        if self.castle_manager.should_create_castle(screen_bounds) {
            let castle_id = self.entity_manager.get_next_id();
            let castle = Castle::new(castle_id, screen_bounds);
            self.entity_manager.add_entity(Box::new(castle));
            self.castle_manager.mark_castle_created(screen_bounds);
        } else if self.castle_manager.should_reposition_castle(screen_bounds) {
            // Remove existing castle and recreate it at new position
            let castle_entities: Vec<_> = self
                .entity_manager
                .get_entities_by_type("castle")
                .iter()
                .map(|e| e.id())
                .collect();

            for castle_id in castle_entities {
                self.entity_manager.remove_entity(castle_id);
            }

            // Create new castle at correct position
            let castle_id = self.entity_manager.get_next_id();
            let castle = Castle::new(castle_id, screen_bounds);
            self.entity_manager.add_entity(Box::new(castle));
        }
    }

    /// Get entity manager reference for rendering
    pub fn entity_manager(&self) -> &EntityManager {
        &self.entity_manager
    }

    /// Initialize water surface if not already done
    fn maybe_initialize_water_surface(&mut self) {
        if !self.water_initialized && self.screen_bounds.width > 0 && self.screen_bounds.height > 0
        {
            let start_id = self.entity_manager.get_next_id();
            let water_layers = self
                .water_surface_manager
                .initialize(self.screen_bounds, start_id);

            // Add water surface layers to entity manager
            for layer in water_layers {
                self.entity_manager.add_entity(Box::new(layer));
            }

            self.water_initialized = true;
        }
    }

    /// Update water surface for screen size changes
    fn update_water_surface(&mut self) {
        if self.water_initialized {
            let layers_updated = self
                .water_surface_manager
                .update_for_screen_size(self.screen_bounds);

            if layers_updated {
                // Remove old water surface entities
                let water_entities: Vec<_> = self
                    .entity_manager
                    .get_entities_by_type("water_surface")
                    .iter()
                    .map(|e| e.id())
                    .collect();

                for id in water_entities {
                    self.entity_manager.remove_entity(id);
                }

                // Add updated water surface layers
                let start_id = self.entity_manager.get_next_id();
                let new_layers = self
                    .water_surface_manager
                    .initialize(self.screen_bounds, start_id);

                for layer in new_layers {
                    self.entity_manager.add_entity(Box::new(layer));
                }
            }
        }
    }
}
