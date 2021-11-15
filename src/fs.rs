use sdl2_sys::{SDL_Free, SDL_GetBasePath, SDL_GetPrefPath};
use std::ffi::CStr;
use camino::Utf8PathBuf as PathBuf;

fn get_wyel_home_wrap(f: impl FnOnce() -> *mut std::os::raw::c_char, dfl: &str) -> PathBuf {
    let x = f();
    if x.is_null() {
        return dfl.to_string();
    }
    unsafe {
        let y = CStr::from_ptr(x);
        // don't call .expect here, to avoid a memory leak
        let z = y.to_str().map(|z| z.to_string());
        std::mem::drop(y);
        SDL_Free(x);
        z
    }
    .expect("SDL_Get*Pref violated its return value docs")
    .into()
}

pub fn home() -> PathBuf {
    get_wyel_home_wrap(
        || unsafe {
            SDL_GetPrefPath(
                CStr::from_bytes_with_nul("ZITE".as_bytes()),
                CStr::from_bytes_with_nul("wyel-sdl".as_bytes()),
            )
        },
        ".",
    )
}

pub fn static_home() -> PathBuf {
    get_wyel_home_wrap(|| unsafe { SDL_GetBasePath() }, "")
}
