use ::Vertex;
use glium::Surface;

pub struct RectangleRenderer {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl RectangleRenderer {
    pub fn new(display: &glium::Display) -> RectangleRenderer {
        let program =
            glium::Program::from_source(
                display,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/rounded_rectangle-vertex.glsl")),
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/rounded_rectangle-frag.glsl")),
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

        RectangleRenderer {
            program,
            vertex_buffer,
            indices,
        }
    }

    pub fn render(&self, target: &mut glium::Frame, matrix: [[f32; 4]; 4], x: f32, y: f32, width: f32, height: f32, color: [f32; 3]) {
        let uniforms1 = uniform! {
            matrix: matrix,
            width: width + 6.0,
            height: height + 6.0,
            radius: 3.0f32,
            x: x - 3.0,
            y: y - 2.0,
            //blur: 5.0f32,
            blur: 7.0f32,
            col: [0.0,0.0,0.0,1.0f32],
            //matrix: Into::<[[f32; 4]; 4]>::into(transform_matrix),
        };
        let uniforms2 = uniform! {
            matrix: matrix,
            width: width,
            height: height,
            radius: 3.0f32,
            x: x,
            y: y,
            blur: 0.0f32,
            col: [color[0],color[1],color[2],1.0f32],
            //matrix: Into::<[[f32; 4]; 4]>::into(transform_matrix),
        };
        //target.clear_color_and_depth((0.5, 0.5, 0.5, 1.0), 1.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms1,
                &glium::DrawParameters {
                    //blend: real_alpha_blending(),
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
        target
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms2,
                &glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    depth: glium::Depth {
                        //test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .unwrap();
    }

    fn render_z(&self, target: &mut glium::Frame, matrix: [[f32; 4]; 4], x: f32, y: f32, z: f32, width: f32, height: f32) {
        let uniforms1 = uniform! {
            matrix: matrix,
            width: width + 6.0*z,
            height: height + 4.0*z,
            radius: 3.0f32,
            x: x - 3.0*z,
            y: y - 1.0*z,
            blur: 5.0f32*z,
            col: [0.0,0.0,0.0,1.0f32],
            //matrix: Into::<[[f32; 4]; 4]>::into(transform_matrix),
        };
        let uniforms2 = uniform! {
            matrix: matrix,
            width: width,
            height: height,
            radius: 3.0f32,
            x: x,
            y: y,
            blur: 0.0f32,
            col: [0.05,0.05,0.05,1.0f32],
            //matrix: Into::<[[f32; 4]; 4]>::into(transform_matrix),
        };
        //target.clear_color_and_depth((0.5, 0.5, 0.5, 1.0), 1.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms1,
                &glium::DrawParameters {
                    //blend: real_alpha_blending(),
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
        target
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms2,
                &glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    depth: glium::Depth {
                        //test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .unwrap();
    }
}
