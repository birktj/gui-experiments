use ::Vertex;
use glium::Surface;
use rusttype::gpu_cache;
use std::cell::RefCell;

pub struct TextRenderer<'a> {
    font: rusttype::Font<'a>,
    cache: RefCell<gpu_cache::Cache<'a>>,
    cache_texture: RefCell<glium::texture::Texture2d>,
	//display: &'a glium::Display,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

// text_renderer.render_text("Open Sans", 20.0, 5.0, 5.0, "Hello world!");
// text_renderer.render_text(5.0, 5.0, "Hello world!");

impl<'a> TextRenderer<'a> {
    pub fn new(display: &glium::Display) -> Self {
        let font = rusttype::Font::from_bytes(&include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/opensans.ttf"))[..]).unwrap();

        let program =
            glium::Program::from_source(
                display,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/text-vertex.glsl")),
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/text-frag.glsl")),
                None).unwrap();

        let mut shape: Vec<Vertex> = Vec::new();
        shape.push(Vertex { position: [1.0, 1.0] });
        shape.push(Vertex { position: [1.0, 0.0] });
        shape.push(Vertex { position: [0.0, 0.0] });

        shape.push(Vertex { position: [0.0, 0.0] });
        shape.push(Vertex { position: [0.0, 1.0] });
        shape.push(Vertex { position: [1.0, 1.0] });

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        //let glyph_texture = glium::texture::Texture2d::new(display, vec![vec![0u8; 1024]; 1024]).unwrap();

        let cache_texture = glium::texture::Texture2d::with_format(
            display,
            glium::texture::RawImage2d {
                data: std::borrow::Cow::Owned(vec![128u8; 1024*1024]),
                width: 1024,
                height: 1024,
                format: glium::texture::ClientFormat::U8,
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap,
        ).unwrap();


        Self {
            font,
            cache: RefCell::new(gpu_cache::Cache::builder().dimensions(1024, 1024).build()),
            cache_texture: RefCell::new(cache_texture),
			// display,
            program,
            vertex_buffer,
            indices,
        }
    }

    pub fn render(&'a self, target: &mut glium::Frame, matrix: [[f32;4];4], x: f32, y: f32, size: f32, color: [f32; 3], text: &str) {
        let glyphs = self.font
            .layout(text, rusttype::Scale::uniform(size), rusttype::Point {x, y})
            .collect::<Vec<_>>();

        for glyph in &glyphs {
            self.cache.borrow_mut().queue_glyph(0, glyph.clone());
        }

        self.cache.borrow_mut().cache_queued(|r, d| {
            self.cache_texture.borrow_mut().main_level().write(
                glium::Rect {
                    left: r.min.x,
                    bottom: r.min.y,
                    width: r.width(),
                    height: r.height(),
                },
                glium::texture::RawImage2d {
                    data: std::borrow::Cow::Borrowed(d),
                    width: r.width(),
                    height: r.height(),
                    format: glium::texture::ClientFormat::U8,
                },
            );
        }).unwrap();

        for glyph in self.font.layout(text, rusttype::Scale::uniform(size), rusttype::Point {x, y}) {
			if let Some((uv_r, sc_r)) = self.cache.borrow().rect_for(0, &glyph).unwrap() {
                let cache_texture = self.cache_texture.borrow();
				let uniforms = uniform! {
					matrix: matrix,
					//width: bb.width() as f32,
					//height: bb.height() as f32,
					width: sc_r.width() as f32,
					height: sc_r.height() as f32,
					x: sc_r.min.x as f32,
					y: sc_r.min.y as f32,
                    t_w: uv_r.width() as f32,
                    t_h: uv_r.height() as f32,
                    t_x: uv_r.min.x as f32,
                    t_y: uv_r.min.y as f32,
					col: [color[0],color[1],color[2],1.0f32],
					tex: cache_texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
				};
				target
					.draw(
						&self.vertex_buffer,
						&self.indices,
						&self.program,
						&uniforms,
						&glium::DrawParameters {
							//blend: real_alpha_blending(),
							blend: glium::Blend::alpha_blending(),
							..Default::default()
						},
					)
					.unwrap();
			}
		}
    }

    pub fn messure(&self, size: f32, text: &str) -> [f32; 2] {
        let v_metrics = self.font.v_metrics(rusttype::Scale::uniform(size));
        let h = v_metrics.ascent - v_metrics.descent;
        let w = self.font
            .layout(text, rusttype::Scale::uniform(size), rusttype::Point {x: 0.0, y: 0.0})
            .last()
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width).unwrap_or(0.0);

        [w, h]
    }
}
