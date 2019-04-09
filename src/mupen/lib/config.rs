use super::types::*;

use libc::{c_char, c_float, c_int, c_void};

#[link(name = "mupen64plus")]
extern "C" {
    pub fn ConfigListSections(
        context: *const c_void,
        section_list_callback: *const extern "C" fn(*const c_void, *const c_char),
    ) -> M64pError;

    pub fn ConfigOpenSection(
        section_name: *const c_char,
        config_section_handle: *const c_void,
    ) -> M64pError;

    pub fn ConfigListParameters(
        config_section_handle: *const c_void,
        context: *const c_void,
        parameter_list_callback: *const extern "C" fn(*const c_void, *const c_char, M64pParamType),
    ) -> M64pError;

    pub fn ConfigHasUnsavedChanges(section_name: *const c_char) -> c_int;

    pub fn ConfigDeleteSection(section_name: *const c_char) -> M64pError;

    pub fn ConfigSaveFile() -> M64pError;

    pub fn ConfigSaveSection(section_name: *const c_char) -> M64pError;

    pub fn ConfigRevertChanges(section_name: *const c_char) -> M64pError;

    pub fn ConfigSetParameter(
        config_section_handle: *const c_void,
        param_name: *const c_char,
        param_type: M64pParamType,
        param_value: *const c_void,
    ) -> M64pError;

    pub fn ConfigGetParameter(
        config_section_handle: *const c_void,
        param_name: *const c_char,
        param_type: M64pParamType,
        param_value: *const c_void,
        max_size: c_int,
    ) -> M64pError;

    pub fn ConfigGetParameterHelp(
        config_section_handle: *const c_void,
        param_name: *const c_char,
    ) -> *const c_char;

    pub fn ConfigSetDefaultInt(
        config_section_handle: *const c_void,
        param_name: *const c_char,
        param_value: *const c_void,
        param_help: *const c_char,
    ) -> M64pError;

    pub fn ConfigSetDefaultFloat(
        config_section_handle: *const c_void,
        param_name: *const c_char,
        param_value: *const c_float,
        param_help: *const c_char,
    ) -> M64pError;

    pub fn ConfigSetDefaultBool(
        config_section_handle: *const c_void,
        param_name: *const c_char,
        param_value: *const c_int,
        param_help: *const c_char,
    ) -> M64pError;

    pub fn ConfigSetDefaultString(
        config_section_handle: *const c_void,
        param_name: *const c_char,
        param_value: *const c_char,
        param_help: *const c_char,
    ) -> M64pError;
}
