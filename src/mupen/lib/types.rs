use libc::{c_int, c_uint};

const CONFIG_VERSION: c_int = 131840;
const DEBUG_VERSION: c_int = 131072;
const VIDEXT_VERSION: c_int = 196608;
const EXTRA_VERSION: c_int = 0;

#[repr(C)]
#[derive(Debug)]
pub enum M64pPluginType {
    Null = 0,
    Rsp = 1,
    Gfx = 2,
    Audio = 3,
    Input = 4,
    Core = 5,
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

#[repr(C)]
pub struct M64p2dSize {
    width: c_uint,
    height: c_uint,
}

#[repr(C)]
pub enum M64pVideoMode {
    None = 1,
    Windowed = 2,
    Fullscreen = 3,
}

#[repr(C)]
pub enum M64pVideoFlags {
    SupportResizing = 1,
}

#[repr(C)]
pub enum M64pParamType {
    Int = 1,
    Float = 2,
    Bool = 3,
    String = 4,
}
