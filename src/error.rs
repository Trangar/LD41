error_chain! {
    foreign_links {
        WindowBuildError(::sdl2::video::WindowBuildError);
        IntegerOrSdlError(::sdl2::IntegerOrSdlError);
        TtfInitError(::sdl2::ttf::InitError);
        NulError(::std::ffi::NulError);
    }
}
