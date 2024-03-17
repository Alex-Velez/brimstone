use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibMode2D, Vector2};

pub trait DottedLine {
    fn draw_dotted_line(
        &mut self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        freqency: f32,
        color: impl Into<Color>,
    );
}

impl DottedLine for RaylibDrawHandle<'_> {
    fn draw_dotted_line(
        &mut self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        freqency: f32,
        color: impl Into<Color>,
    ) {
        let start: Vector2 = start_pos.into();
        let end: Vector2 = end_pos.into();
        let color = color.into();

        let distance = start.distance_to(end);
        let gaps = distance / freqency;
        let total_change = end - start;
        let step = total_change / gaps;

        for t in 0..gaps as u32 {
            let displace = step * t as f32;
            self.draw_circle_v(start + displace, 1.0, color);
        }
    }
}

impl DottedLine for RaylibMode2D<'_, RaylibDrawHandle<'_>> {
    fn draw_dotted_line(
        &mut self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        freqency: f32,
        color: impl Into<Color>,
    ) {
        let start: Vector2 = start_pos.into();
        let end: Vector2 = end_pos.into();
        let color = color.into();

        let distance = start.distance_to(end);
        let gaps = distance / freqency;
        let total_change = end - start;
        let step = total_change / gaps;

        for t in 0..gaps as u32 {
            let displace = step * t as f32;
            self.draw_circle_v(start + displace, 1.0, color);
        }
    }
}
