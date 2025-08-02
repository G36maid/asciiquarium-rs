use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::app::App;

impl Widget for &App {
    /// Renders the aquarium with all entities
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear the screen with default/transparent background
        // Let entities handle their own background colors
        for y in 0..area.height {
            for x in 0..area.width {
                if x < buf.area.width && y < buf.area.height {
                    let cell = buf.cell_mut((x, y)).unwrap();
                    cell.set_char(' ');
                    cell.set_style(Style::default()); // Transparent background everywhere
                }
            }
        }

        // Water surface is now rendered through the entity system

        // Render all entities through the entity manager
        self.entity_manager().render_all(buf, area);

        // Render status information
        self.render_status(area, buf);
    }
}

impl App {
    /// Render status information
    fn render_status(&self, area: Rect, buf: &mut Buffer) {
        let fish_count = self.entity_manager().get_entities_by_type("fish").len();
        let bubble_count = self.entity_manager().get_entities_by_type("bubble").len();
        let water_count = self
            .entity_manager()
            .get_entities_by_type("water_surface")
            .len();
        let total_entities = self.entity_manager().entity_count();

        // Get debug info about first fish position
        let fish_debug =
            if let Some(first_fish) = self.entity_manager().get_entities_by_type("fish").first() {
                let pos = first_fish.position();
                format!("Fish1@({:.1},{:.1})", pos.x, pos.y)
            } else {
                "NoFish".to_string()
            };

        let status_line = if self.paused {
            format!(
                "PAUSED | Fish: {} | Bubbles: {} | Water: {} | {} | Total: {} | q=quit r=redraw p=pause",
                fish_count, bubble_count, water_count, fish_debug, total_entities
            )
        } else {
            format!(
                "Fish: {} | Bubbles: {} | Water: {} | {} | Total: {} | q=quit r=redraw p=pause",
                fish_count, bubble_count, water_count, fish_debug, total_entities
            )
        };

        // Render status at the bottom
        let status_y = area.height.saturating_sub(1);
        for (x, ch) in status_line.chars().enumerate().take(area.width as usize) {
            if (x as u16) < buf.area.width && status_y < buf.area.height {
                let cell = buf.cell_mut((x as u16, status_y)).unwrap();
                cell.set_char(ch);
                cell.set_style(Style::default().fg(Color::White).bg(Color::Black));
            }
        }
    }
}
