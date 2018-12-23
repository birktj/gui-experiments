#[macro_use]
extern crate glium;
extern crate nalgebra as na;
extern crate rusttype;

use std::io::Write;

use glium::{glutin, Surface};
use na::{Matrix4, Rotation3, Similarity3, Transform3, UnitQuaternion, Vector3};

mod render;
//mod api;

mod example;

use render::*;

#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub fn real_alpha_blending() -> glium::Blend {
    glium::Blend {
        color: glium::BlendingFunction::Addition {
            source: glium::LinearBlendingFactor::Zero,
            destination: glium::LinearBlendingFactor::SourceAlpha,
        },
        alpha: glium::BlendingFunction::Addition {
            source: glium::LinearBlendingFactor::Zero,
            destination: glium::LinearBlendingFactor::One,
        },
        constant_value: (0.0, 0.0, 0.0, 0.0)
    }
}

pub trait Drawing {
    //fn layout(&self, ctx: &mut LayoutContext);

    fn draw(&self, ctx: &mut RenderContext);
}

pub trait View {
	type D: Drawing;
    fn view(&self) -> Self::D;
}

impl<V: View> Drawing for V {
    fn draw(&self, ctx: &mut RenderContext) {
        self.view().draw(ctx);
    }
}

struct Renderer {
    rectangle_program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}


struct RenderState {
    colors: Vec<[f32; 3]>,
}

impl RenderState {
    fn from_state(state: i32) -> RenderState {
        let mut colors = Vec::new();
        for i in 0..20 {
            let color = if i == state {
                [0.03, 0.04, 0.07]
            }
            else {
                [0.03, 0.03, 0.03]
            };
            colors.push(color);
        }

        RenderState {
            colors
        }
    }

    fn step_towards(&mut self, goal: &RenderState) {
        for i in 0..20 {
            for j in 0..3 {
                self.colors[i][j] += (goal.colors[i][j] - self.colors[i][j]) / 10.0;
            }
        }
    }
}

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new()
        //.with_multisampling(8); // fucks up alpha ??? 
        .with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let font = rusttype::Font::from_bytes(&include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/opensans.ttf"))[..]).unwrap();

    let glyph = font.glyph('A').scaled(rusttype::Scale::uniform(100.0)).positioned(rusttype::Point {x: 0.0, y: 0.0});

    let mut glyph_buffer = vec![vec![0.0; 100]; 100];
    glyph.draw(|x, y, c| {
        if x < 100 && y < 100 {
            glyph_buffer[y as usize][x as usize] = c;
        }
    });

    let rectangle_renderer = RectangleRenderer::new(&display);
    let mut text_renderer = TextRenderer::new(&display);


    let window_size = display.gl_window().window().get_inner_size().unwrap();

    let mut transform_matrix = Transform3::identity() * Similarity3::from_scaling(0.01);

    let mut focus = 0;

    let mut curr_state = RenderState::from_state(focus);
    let mut next_state = RenderState::from_state(focus);

    let mut closed = false;
	
	print!("fps: ");

	//let rec = api::Rectangle::new(100.0, 100.0);
	
	//let inner_rec = api::Rectangle::new(100.0, 100.0);
	/*
	let inner = api::Label::new(&text_renderer, "Hello world!".to_string());
	let outer_rec = api::Rectangle::new_with_child(inner.as_widget());

	//inner_rec.reposition(150.0, 150.0);
	outer_rec.reposition(100.0, 100.0);
	*/

	let mut state = example::State::new();

	let mut now = std::time::SystemTime::now();

    while !closed {
		print!("\rfps: {:?}", now.elapsed());
		std::io::stdout().flush().unwrap();
		now = std::time::SystemTime::now();
        next_state = RenderState::from_state(focus);
        curr_state.step_towards(&next_state);
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        let matrix =  [
                [2.0 / width as f32, 0.0, 0.0, -1.0],
                [0.0, 2.0 / height as f32, 0.0, 1.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ];

		state.layout(width as f32, height as f32);

		{
			let mut ctx = RenderContext::new(&mut target, &rectangle_renderer, &text_renderer);
			
			/*
			ctx.rectangle(50.0, 50.0, width as f32 - 100.0, 100.0);
			ctx.text(100.0, 100.0, "Hello world!");
			*/

			ctx.clear([0.02; 3]);
			//outer_rec.draw(&mut ctx);
		}
		
		/*
		target.clear_color(0.02,0.02,0.02,1.0);
		rectangle_renderer.render(&mut target, matrix, 50.0, 50.0, width as f32 - 100.0, 250.0, [0.03, 0.03, 0.03]);
		rectangle_renderer.render(&mut target, matrix, 70.0, 70.0, width as f32 - 140.0, 150.0, [0.03, 0.03, 0.03]);

		text_renderer.render(&mut target, matrix, 100.0, 100.0, 15.0, [1.0, 1.0, 1.0], "Hello world!");
		*/

        /*
        for i in 0..20 {
            let color = curr_state.colors[i];
            rectangle_renderer.render(&mut target, matrix, 100.0, 100.0 + 40.0 * i as f32, width as f32 - 200.0, 25.0, color);
        }*/

		{
			let mut ctx = RenderContext::new(&mut target, &rectangle_renderer, &text_renderer);
			
			state.render(&mut ctx);
		}
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            //println!("Event: {:?}", ev);
            if let Some(window_size) = display.gl_window().window().get_inner_size() {
                // Resize here
            }
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput { input, ..} => {
                        match (input.virtual_keycode, input.state) {
                            (Some(glutin::VirtualKeyCode::Down), glutin::ElementState::Pressed) => state.down(),
                            (Some(glutin::VirtualKeyCode::Up), glutin::ElementState::Pressed) => state.up(),
                            (Some(glutin::VirtualKeyCode::Right), glutin::ElementState::Pressed) => state.right(),
                            (Some(glutin::VirtualKeyCode::Left), glutin::ElementState::Pressed) => state.left(),
                            _ => (),
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
	println!("");
}
