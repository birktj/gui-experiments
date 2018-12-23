use glium::Surface;

mod rectangle;
mod text;
pub use self::rectangle::RectangleRenderer;
pub use self::text::TextRenderer;

pub struct RenderContext<'a, 'b> {
    frame: &'b mut glium::Frame,
    rectangle_renderer: &'a RectangleRenderer,
	text_renderer: &'a TextRenderer<'a>,
    matrix: [[f32; 4]; 4],
    size: [f32; 2],
}

impl<'a, 'b> RenderContext<'a, 'b> {
    pub fn new(frame: &'b mut glium::Frame, rectangle_renderer: &'a RectangleRenderer, text_renderer: &'a TextRenderer<'a>) -> Self {
        let (width, height) = frame.get_dimensions();
        let matrix =  [
                [2.0 / width as f32, 0.0, 0.0, -1.0],
                [0.0, 2.0 / height as f32, 0.0, 1.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ];

        Self {
            frame,
            rectangle_renderer,
			text_renderer,
            matrix,
            size: [width as f32, height as f32],
        }
    }

    pub fn size(&self) -> [f32; 2] {
        self.size
    }

    pub fn width(&self) -> f32 {
        self.size[0]
    }

    pub fn height(&self) -> f32 {
        self.size[1]
    }

	/*
    fn with_context<F: Fn(&mut RenderContext<'a>)>(frame: &'a mut glium::Frame, rec_render: &'a RectangleRenderer, f: F) {
        let mut ctx = RenderContext::new(frame, rec_render);
        f(&mut ctx);
    }*/

    pub fn clear(&mut self, color: [f32; 3]) {
        self.frame.clear_color(color[0], color[1], color[2], 1.0);
    }

    //pub fn clear_rectangle // TODO: redraw only subrectangle

    pub fn rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 3]) {
        self.rectangle_renderer.render(&mut self.frame, self.matrix, x, y, w, h, color);
    }

    pub fn text(&mut self, x: f32, y: f32, text: &str, color: [f32; 3]) {
        self.text_renderer.render(&mut self.frame, self.matrix, x, y, 20.0, color, text);
    }

    pub fn messure_text(&mut self, text: &str) -> [f32; 2] {
        self.text_renderer.messure(20.0, text)
    }
}
