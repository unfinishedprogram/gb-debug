use egui::{ColorImage, Image, TextureHandle, TextureOptions, Ui, Vec2};

#[derive(Default)]
pub struct Screen {
    texture: Option<TextureHandle>,
}

impl Screen {
    pub fn draw(&mut self, ui: &mut Ui, buffer: &[u8]) {
        let image = ColorImage::from_rgba_unmultiplied([160, 144], buffer);

        let texture = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("screen", image.clone(), TextureOptions::NEAREST)
        });

        texture.set(image, TextureOptions::NEAREST);

        ui.add(Image::new(
            texture.id(),
            Vec2::new(160.0 * 4.0, 144.0 * 4.0),
        ));
    }
}
