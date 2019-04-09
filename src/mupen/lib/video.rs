use super::types::*;

use libc::{c_char, c_int};

#[link(name = "mupen64plus")]
extern "C" {
    pub fn VidExt_Init() -> M64pError;

    pub fn VidExt_Quit() -> M64pError;

    pub fn VidExt_ListFullscreenModes(
        size_array: *const M64p2dSize,
        num_sizes: *const c_int,
    ) -> M64pError;

    pub fn VidExt_SetVideoMode(
        width: c_int,
        height: c_int,
        bits_per_pixel: c_int,
        video_mode: M64pVideoMode,
        video_flags: M64pVideoFlags,
    ) -> M64pError;

    pub fn VidExt_SetCaption(title: *const c_char) -> M64pError;

    pub fn VidExt_ToggleFullScreen() -> M64pError;

    pub fn VidExt_ResizeWindow(width: c_int, height: c_int) -> M64pError;
}
