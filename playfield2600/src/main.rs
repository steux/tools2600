use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui, Skin
};

#[derive(Debug)]
pub enum Register {
    Colubk,
    Colupf,
    Ctrlpf,
    Pf0,
    Pf1,
    Pf2
}

type Playfield = Vec<(Register, u8)>;

#[macroquad::main("Playfield2600")]
async fn main() {
    let mut playfield = Playfield::new();
    playfield.push((Register::Colubk, 0));
    playfield.push((Register::Colupf, 0));

    let label_style = root_ui()
        .style_builder()
        .font(include_bytes!("./AtariClassic-Regular.ttf"))
        .unwrap()
        .text_color(Color::from_rgba(0, 0, 0, 255))
        .font_size(16)
        .build();

    let skin = Skin { label_style,
        ..root_ui().default_skin()};

    loop {
        clear_background(WHITE);
        root_ui().push_skin(&skin);

        widgets::Window::new(hash!(), vec2(0., 0.), vec2(210., 400.))
            .label("Playfield definition")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                for i in &playfield {
                    Group::new(hash!(), Vec2::new(200., 20.)).ui(ui, |ui| {
                        ui.label(Vec2::new(2., 2.), &format!("{:?}", i.0));
                    });
                }
            });

        widgets::Window::new(hash!(), vec2(220., 0.), vec2(640., 480.))
            .label("Preview")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
            });

        next_frame().await;
    }
}
