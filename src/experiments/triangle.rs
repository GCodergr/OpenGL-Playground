extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::video::GLProfile;

use std::time::Duration;
use std::ffi::CString;

use crate::experiments::render_utilities;

const SCREEN_WIDTH : u32 = 800;
const SCREEN_HEIGHT : u32 = 600;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("OpenGL Playground", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(GLProfile::Core);
    gl_attributes.set_context_version(3, 3);

    let _gl_context = window.gl_create_context()?;
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    debug_assert_eq!(gl_attributes.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attributes.context_version(), (3, 3));

    let vert_shader = render_utilities::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader = render_utilities::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render_utilities::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vertices: Vec<f32> = vec![
        // positions        // color
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,   // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0,   // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
        gl::VertexAttribPointer(
            1,         // index of the generic vertex attribute ("layout (location = 1)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0); // or (0.3, 0.3, 0.3, 1.0)
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

        window.gl_swap_window();

        // We sleep enough to get ~60 fps. If we don't call this, the program will take 100% of a CPU time.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}