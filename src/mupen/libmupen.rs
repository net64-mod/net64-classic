use libc::{c_char, c_int, c_void};

#[link(name = "mupen64plus")]
extern "C" {
    pub fn CoreStartup(
        api_version: c_int,
        config_path: *const c_char,
        data_path: *const c_char,
        context: *const c_void,
        debug_callback: *const c_void,
        // debug_callback: *const Fn<(*const c_void, c_int, *const char)>,
        context2: *const c_void,
        state_callback: *const c_void,
        // state_callback: *const Fn<(*const c_void, c_int, c_int)>,
    ) -> M64pError;

    pub fn CoreGetAPIVersions(
        config_version: *mut c_int,
        debug_version: *mut c_int,
        vidext_version: *mut c_int,
        extra_version: *mut c_int,
    ) -> M64pError;

    pub fn CoreDoCommand(command: M64pCommand, param_int: c_int, param_ptr: *const c_void)
        -> M64pError;

    pub fn CoreErrorMessage(m64p_error: M64pError) -> *const c_char;
}

#[repr(C)]
pub enum M64pCommand {
    Nop = 0,
    RomOpen = 1,
    RomClose = 2,
    RomGetHeader = 3,
    RomGetSettings = 4,
    Execute = 5,
    Stop = 6,
    Pause = 7,
    Resume = 8,
    CoreStateQuery = 9,
    StateLoad = 10,
    StateSave = 11,
    StateSetSlot = 12,
    SendSdlKeydown = 13,
    SendSdlKeyup = 14,
    SetFrameCallback = 15,
    TakeNextScreenshot = 16,
    CoreStateSet = 17,
    ReadScreen = 18,
    Reset = 19,
    AdvanceFrame = 20,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum M64pError {
    Success = 0,
    NotInit = 1,
    AlreadyInit = 2,
    Incompatible = 3,
    InputAssert = 4,
    InputInvalid = 5,
    InputNotFound = 6,
    NoMemory = 7,
    Files = 8,
    Internal = 9,
    InvalidState = 10,
    PluginFail = 11,
    SystemFail = 12,
    Unsupported = 13,
    WrongType = 14,
}