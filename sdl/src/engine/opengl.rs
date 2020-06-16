
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
}

fn load_gl_function(video_sub: &sdl2::VideoSubsystem) {
    gl::load_with(
        |s| video_sub.gl_get_proc_address(s) 
        as *const std::os::raw::c_void
    );
}