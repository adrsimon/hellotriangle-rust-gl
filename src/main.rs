extern crate gl;
extern crate glfw;

use core::str;
use std::{ffi::CString, ptr, mem};

use glfw::{Action, Context, Key};

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 position;
    layout (location = 1) in vec3 color;
    out vec3 ourColor;
    void main() {
        gl_Position = vec4(position.x, position.y, position.z, 1.0);
        ourColor = color;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    in vec3 ourColor;
    out vec4 color;
    void main() {
        color = vec4(ourColor, 1.0f);
    }
"#;

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Resizable(true));

    let (mut window, events) = glfw.create_window(
        WIDTH, 
        HEIGHT, 
        "Hello Triangle from Rust GL", 
        glfw::WindowMode::Windowed
    ).expect("Failed to create window.");
    
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
         0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
         0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let (shader_program, vao) = unsafe {
        let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len: gl::types::GLint = 0;
            gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                shader_program,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut gl::types::GLchar,
            );
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&buffer).unwrap());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let mut vbo = 0;
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );


        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, (3 * mem::size_of::<gl::types::GLfloat>()) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);


        (shader_program, vao)
    }; 

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }
    }
}

unsafe fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> gl::types::GLuint {
    let shader = gl::CreateShader(shader_type);
    let c_str = CString::new(source.as_bytes()).unwrap();

    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader);
    
    let mut success = gl::FALSE as gl::types::GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success != gl::TRUE as gl::types::GLint {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = Vec::with_capacity(len as usize);
        buffer.set_len((len as usize) - 1);
        gl::GetShaderInfoLog(
            shader,
            len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut gl::types::GLchar,
        );
        panic!("ERROR::SHADER::COMPILATION_FAILED\n{}", str::from_utf8(&buffer).unwrap());
    }

    shader
}
