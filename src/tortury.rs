
use glium::{glutin::surface::WindowSurface, Program, Surface, Texture2d, VertexBuffer};
use image::Rgba;
use winit::{dpi::LogicalPosition, event_loop::EventLoop, window::Window};
use glium::backend::glutin::Display;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct OknoWroga{
    x: f32, 
    y: f32, 
    speed: f32, 
    mov_x : f32,
    mov_y : f32,


    width: u32,
    height: u32,
    pub window: Window,
    pub display: Display<WindowSurface>,

    vertex_buffer : VertexBuffer<Vertex>,
    indices : glium::index::NoIndices,
    program : Program, 
    texture: Texture2d,
}

impl OknoWroga{
    pub fn new(image: image::ImageBuffer<Rgba<u8>, Vec<u8>>, width: u32, height: u32, speed: f32, event_loop : &EventLoop<()>) -> Self{
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Zaczynamy!")
            .with_inner_size(width, height)
            .build(&event_loop);

        let shape = vec![
            Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
            Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
            Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },

            Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
            Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
            Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        ];

        let indices: glium::index::NoIndices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let program = create_program(&display);

        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::Texture2d::new(&display, image).unwrap();

        OknoWroga{x: 0.0, y: 0.0, speed: speed, mov_x: speed, mov_y: speed, width: width, height: height, window: window, display: display, vertex_buffer: vertex_buffer, indices: indices, program: program, texture: texture}
    }

    pub fn draw(&self){
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [ 0.0 , 0.0, 0.0, 1.0f32],
            ],
            tex: &self.texture,
        };

        target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }

    pub fn update(&mut self){
        self.window.set_outer_position(LogicalPosition::new(self.x, self.y));

        if self.x > 1920.0 - self.width as f32 {
            self.mov_x = -self.speed;
        }
        else if self.x < 0.0 {
            self.mov_x = self.speed;
        }

        if self.y > 1080.0 - self.height as f32 {
            self.mov_y = -self.speed;
        }
        else if self.y < 0.0 {
            self.mov_y = self.speed;
        }

        self.x+=self.mov_x;
        self.y+=self.mov_y;
        self.window.focus_window();
    }
}

pub fn create_program(display : &Display<WindowSurface>) -> Program {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}