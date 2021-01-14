use eframe::egui::{
    math::Vec2,
    paint::{
        color::Color32,
        command::{PaintCmd, Stroke},
    },
    Response, Sense, Ui,
};

pub struct ChessBoard {
    cells_size: f32,
}

impl ChessBoard {
    pub fn new(cells_size: f32) -> Self {
        Self { cells_size }
    }

    pub fn draw(&mut self, ui: &mut Ui) -> Response {
        let total_size = 9f32 * self.cells_size;
        let (response, painter) =
            ui.allocate_painter(Vec2{x: total_size, y: total_size}, Sense::drag());
        let rect = response.rect;

        painter.add(PaintCmd::Rect {
            rect,
            corner_radius: 0f32,
            stroke: Stroke {
                width: 0f32,
                color: Color32::TRANSPARENT,
            },
            fill: Color32::from_rgb(214, 59, 96),
        });

        response
    }
}
