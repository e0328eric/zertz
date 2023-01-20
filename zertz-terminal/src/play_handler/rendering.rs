use crossterm::{
    event::{KeyEvent, MouseEvent},
    style::Stylize,
};

use crate::{coordinate::Coordinate, error, terminal_handler::titlebox::TitleBox};

use super::PlayHandler;

pub const PRIMARY_TEXT_OFFSET: u16 = 14;
pub const SUPPLIMENTARY_TEXT_OFFSET: u16 = 15;

impl PlayHandler {
    pub fn render_game(
        &mut self,
        key_event: Option<KeyEvent>,
        mouse_event: Option<MouseEvent>,
    ) -> error::Result<()> {
        // Begin Drawing
        self.terminal_handler.clear()?;

        let Coordinate {
            x: center_x,
            y: center_y,
        } = self.terminal_handler.center;

        self.terminal_handler.draw_object(
            &"Zertz Board Game".bold(),
            center_x - 8,
            center_y - 24,
        )?;
        self.terminal_handler
            .clear_line(self.center.y + PRIMARY_TEXT_OFFSET)?;
        self.terminal_handler
            .clear_line(self.center.y + SUPPLIMENTARY_TEXT_OFFSET)?;
        self.terminal_handler.draw_object(
            &self.explain_primary_text,
            self.center.x - self.explain_primary_text.len() as u16 / 2,
            self.center.y + PRIMARY_TEXT_OFFSET,
        )?;
        self.terminal_handler.draw_object(
            &self.explain_supplimentary_text,
            self.center.x - self.explain_supplimentary_text.len() as u16 / 2,
            self.center.y + SUPPLIMENTARY_TEXT_OFFSET,
        )?;
        self.terminal_handler.draw_board(&self.game_board)?;

        #[cfg(debug_assertions)]
        {
            let width = self.center.x * 2;
            let height = self.center.y * 2;
            self.terminal_handler.draw_shape(
                &TitleBox::new(width - 98, 0, 98, 8, "< Debug >")
                    .bold()
                    .blue(),
            )?;
            self.terminal_handler.draw_shape(
                &TitleBox::new(width - 97, 1, 96, 3, "< Keyboard >")
                    .bold()
                    .red(),
            )?;
            self.terminal_handler.draw_shape(
                &TitleBox::new(width - 97, 4, 96, 3, "< Mouse >")
                    .bold()
                    .red(),
            )?;
            self.terminal_handler.draw_shape(
                &TitleBox::new(0, height - 11, width, 11, "< Debug >")
                    .bold()
                    .blue(),
            )?;
            self.terminal_handler.draw_shape(
                &TitleBox::new(1, height - 10, 30, 3, "< PlayHandler State >")
                    .bold()
                    .green(),
            )?;
            self.terminal_handler.draw_shape(
                &TitleBox::new(32, height - 10, 30, 3, "< Game State >")
                    .bold()
                    .green(),
            )?;
            self.terminal_handler.draw_shape(
                &TitleBox::new(1, height - 7, width - 2, 3, "< Input Data >")
                    .bold()
                    .green(),
            )?;

            self.terminal_handler.draw_object(
                &format!("{:<94}", format!("{:?}", key_event)),
                width - 96,
                2,
            )?;
            self.terminal_handler.draw_object(
                &format!("{:<94}", format!("{:?}", mouse_event)),
                width - 96,
                5,
            )?;
            self.terminal_handler
                .draw_object(&format!("{:?}", &self.state), 2, height - 9)?;
            self.terminal_handler.draw_object(
                &format!("{:?}", &self.app.get_game_state()),
                33,
                height - 9,
            )?;
            self.terminal_handler.draw_object(
                &format!("{:<94}", format!("{:?}", &self.input_data)),
                2,
                height - 6,
            )?;
        }

        // End Drawing
        self.terminal_handler.flush()?;

        Ok(())
    }
}
