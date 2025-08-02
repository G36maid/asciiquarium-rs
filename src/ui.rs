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
        // Clear the screen with a dark blue ocean background
        for y in 0..area.height {
            for x in 0..area.width {
                if x < buf.area.width && y < buf.area.height {
                    let cell = buf.cell_mut((x, y)).unwrap();
                    cell.set_char(' ');
                    cell.set_style(Style::default().bg(Color::Blue));
                }
            }
        }

        // Render water surface at the top
        self.render_water_surface(area, buf);

        // Render all entities through the entity manager
        self.entity_manager().render_all(buf, area);

        // Render status information
        self.render_status(area, buf);
    }
}

impl App {
    /// Render the water surface animation
    fn render_water_surface(&self, area: Rect, buf: &mut Buffer) {
        let water_segments = [
            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
            "^^^^ ^^^  ^^^   ^^^    ^^^^      ",
            "^^^^      ^^^^     ^^^    ^^     ",
            "^^      ^^^^      ^^^    ^^^^^^  ",
        ];

        for (i, segment) in water_segments.iter().enumerate() {
            let y = 5 + i as u16;
            if y < area.height {
                // Tile the segment across the screen width
                let segment_len = segment.len();
                let repeats = (area.width as usize / segment_len) + 1;
                let tiled_segment = segment.repeat(repeats);

                for (x, ch) in tiled_segment.chars().enumerate().take(area.width as usize) {
                    if (x as u16) < buf.area.width && y < buf.area.height {
                        let cell = buf.cell_mut((x as u16, y)).unwrap();
                        cell.set_char(ch);
                        cell.set_style(Style::default().fg(Color::Cyan).bg(Color::Blue));
                    }
                }
            }
        }
    }

    /// Render status information
    fn render_status(&self, area: Rect, buf: &mut Buffer) {
        let fish_count = self.entity_manager().get_entities_by_type("fish").len();
        let total_entities = self.entity_manager().entity_count();

        let status_line = if self.paused {
            format!(
                "PAUSED | Fish: {} | Total: {} | q=quit r=redraw p=pause",
                fish_count, total_entities
            )
        } else {
            format!(
                "Fish: {} | Total: {} | q=quit r=redraw p=pause",
                fish_count, total_entities
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
