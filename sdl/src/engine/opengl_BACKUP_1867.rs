
extern crate gl;
extern crate sdl2;

pub struct SDL_OpenGL {
    inited:     bool
}

impl SDL_OpenGL {
    pub fn init(video_sub: &sdl2::VideoSubsystem) -> SDL_OpenGL {
        load_gl_function(video_sub);
        SDL_OpenGL {
            inited: true
        }
    }

    pub fn set_clear_color(&self) {
        if self.inited {
            unsafe {
                gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            }
        }
    }

    pub fn clear(&self) {
<<<<<<< HEAD
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
=======
        if self.inited {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
>>>>>>> 4267232dddd3f5f925ecf0ac81bfdca3d655da7a
        }
    }
}

fn load_gl_function(video_sub: &sdl2::VideoSubsystem) {
    gl::load_with(
        |s| video_sub.gl_get_proc_address(s) 
        as *const std::os::raw::c_void
    );
}