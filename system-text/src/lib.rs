mod error;
use std::collections::HashMap;

pub struct SystemTextRenderer {
    program: u32,
    vao: u32,
    buffer: u32,
    num_indices: usize,

    character_vertices: HashMap<char, Vec<f32>>,
}

pub struct TextLine {
    pub text: String,
    pub position: (f32, f32),
    pub char_size: (f32, f32),
}

impl SystemTextRenderer {
    pub fn new() -> Self {
        Self {
            program: 0,
            vao: 0,
            buffer: 0,
            num_indices: 0,
            character_vertices: HashMap::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), error::Error> {
        self.program = create_program()?;
        self.character_vertices = create_character_vertices();
        (self.vao, self.buffer) = create_vertex_array()?;

        Ok(())
    }

    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::LINES, 0, self.num_indices as i32);
        }
    }

    pub fn set_text(&mut self, text: &[TextLine]) {
        let char_scale = (0.8, 0.7);
        unsafe {
            let mut vertices = Vec::new();
            for t in text {
                let char_scale = (char_scale.0 * t.char_size.0, char_scale.1 * t.char_size.1);
                let mut char_start = t.position;
                for c in t.text.chars() {
                    if let Some(v) = self.character_vertices.get(&c) {
                        for i in 0..v.len() / 2 {
                            let index = i * 2;
                            let vx = char_start.0 + v[index] * char_scale.0;
                            let vy = char_start.1 - v[index + 1] * char_scale.1;
                            vertices.push(vx);
                            vertices.push(vy);
                        }
                    }
                    char_start.0 += t.char_size.0;

                    if c == '\n' {
                        char_start.0 = t.position.0;
                        char_start.1 -= t.char_size.1;
                    }
                }
            }

            self.num_indices = vertices.len();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }
}

fn create_character_vertices() -> HashMap<char, Vec<f32>> {
    let mut vertices = HashMap::new();

    let v = vec![0.0, 1.0, 0.5, 0.0, 0.5, 0.0, 1.0, 1.0, 0.25, 0.5, 0.75, 0.5];
    vertices.insert('A', v);

    let v = vec![
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.5, 0.5, 0.5,
    ];
    vertices.insert('E', v);

    let v = vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.5, 1.0, 0.5];
    vertices.insert('H', v);

    let v = vec![0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0];
    vertices.insert('L', v);

    let v = vec![
        0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    ];
    vertices.insert('O', v);

    vertices
}

fn create_vertex_array() -> Result<(u32, u32), error::Error> {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut buffer = 0;
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);

        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (2 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(0);

        Ok((vao, buffer))
    }
}

fn create_program() -> Result<u32, error::Error> {
    unsafe {
        let vshader_code = include_str!("shaders/vshader.glsl");
        let fshader_code = include_str!("shaders/fshader.glsl");

        let vshader = create_shader(vshader_code, gl::VERTEX_SHADER)?;
        let fshader = create_shader(fshader_code, gl::FRAGMENT_SHADER)?;

        let program = gl::CreateProgram();
        gl::AttachShader(program, vshader);
        gl::AttachShader(program, fshader);
        gl::LinkProgram(program);

        let mut status = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = vec![0; len as usize];
            gl::GetProgramInfoLog(program, len, &mut len, buf.as_mut_ptr() as *mut _);

            let message = std::str::from_utf8(&buf).unwrap().to_string();
            Err(error::LinkError::new(message).into())
        } else {
            Ok(program)
        }
    }
}

fn create_shader(shader_code: &str, shader_type: u32) -> Result<u32, error::CompileError> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(
            shader,
            1,
            &(shader_code.as_ptr() as *const _),
            &(shader_code.len() as i32),
        );
        gl::CompileShader(shader);

        let mut status = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = vec![0; len as usize];
            gl::GetShaderInfoLog(shader, len, &mut len, buf.as_mut_ptr() as *mut _);

            let message = std::str::from_utf8(&buf).unwrap().to_string();
            Err(error::CompileError::new(message))
        } else {
            Ok(shader)
        }
    }
}
