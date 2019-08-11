
use super::types::*;
use super::Mupen64Plus;

use libc::{c_char, c_int};
use libloading::Symbol;

#[allow(non_camel_case_types)]
type VidExt_Init = unsafe fn() -> M64pError;

#[allow(non_camel_case_types)]
type VidExt_Quit = unsafe fn() -> M64pError;

#[allow(non_camel_case_types)]
type VidExt_ListFullscreenModes = unsafe fn(*const M64p2dSize, *const c_int) -> M64pError;

#[allow(non_camel_case_types)]
type VidExt_SetVideoMode =
    unsafe fn(c_int, c_int, c_int, M64pVideoMode, M64pVideoFlags) -> M64pError;

#[allow(non_camel_case_types)]
type VidExt_SetCaption = unsafe fn(title: *const c_char) -> M64pError;

#[allow(non_camel_case_types)]
type VidExt_ToggleFullScreen = unsafe fn() -> M64pError;

#[allow(non_camel_case_types)]
type VidExt_ResizeWindow = unsafe fn(width: c_int, height: c_int) -> M64pError;

impl Mupen64Plus {
    pub fn vid_ext_init(&self) -> M64pError {
        unsafe {
            let vid_ext_init: Symbol<VidExt_Init> = self.lib.get(b"VidExt_Init").unwrap();

            vid_ext_init()
        }
    }

    pub fn vid_ext_quit(&self) -> M64pError {
        unsafe {
            let vid_ext_quit: Symbol<VidExt_Quit> = self.lib.get(b"VidExt_Quit").unwrap();

            vid_ext_quit()
        }
    }

    pub fn vid_ext_list_fullscreen_modes(
        &self,
        size_array: *const M64p2dSize,
        num_sizes: *const c_int,
    ) -> M64pError {
        unsafe {
            let vid_ext_list_fullscreen_modes: Symbol<VidExt_ListFullscreenModes> =
                self.lib.get(b"VidExt_ListFullscreenModes").unwrap();

            vid_ext_list_fullscreen_modes(size_array, num_sizes)
        }
    }

    pub fn vid_ext_set_video_mode(
        &self,
        width: c_int,
        height: c_int,
        bits_per_pixel: c_int,
        video_mode: M64pVideoMode,
        video_flags: M64pVideoFlags,
    ) -> M64pError {
        unsafe {
            let vid_ext_set_video_mode: Symbol<VidExt_SetVideoMode> =
                self.lib.get(b"VidExt_SetVideoMode").unwrap();

            vid_ext_set_video_mode(width, height, bits_per_pixel, video_mode, video_flags)
        }
    }

    pub fn vid_ext_set_caption(&self, title: *const c_char) -> M64pError {
        unsafe {
            let vid_ext_set_caption: Symbol<VidExt_SetCaption> =
                self.lib.get(b"VidExt_SetCaption").unwrap();

            vid_ext_set_caption(title)
        }
    }

    pub fn vid_ext_toggle_fullscreen(&self) -> M64pError {
        unsafe {
            let vid_ext_toggle_fullscreen: Symbol<VidExt_ToggleFullScreen> =
                self.lib.get(b"VidExt_ToggleFullScreen").unwrap();

            vid_ext_toggle_fullscreen()
        }
    }

    pub fn vid_ext_resize_window(&self, width: c_int, height: c_int) -> M64pError {
        unsafe {
            let vid_ext_resize_window: Symbol<VidExt_ResizeWindow> =
                self.lib.get(b"VidExt_ResizeWindow").unwrap();

            vid_ext_resize_window(width, height)
        }
    }
}
