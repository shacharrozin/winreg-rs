// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.

//! `use winreg::enums::*;` to import all needed enumerations and constants
pub use windows_sys::Win32::System::Registry::{
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_CURRENT_USER_LOCAL_SETTINGS,
    HKEY_DYN_DATA, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_PERFORMANCE_NLSTEXT,
    HKEY_PERFORMANCE_TEXT, HKEY_USERS, KEY_ALL_ACCESS, KEY_CREATE_LINK, KEY_CREATE_SUB_KEY,
    KEY_ENUMERATE_SUB_KEYS, KEY_EXECUTE, KEY_NOTIFY, KEY_QUERY_VALUE, KEY_READ, KEY_SET_VALUE,
    KEY_WOW64_32KEY, KEY_WOW64_64KEY, KEY_WOW64_RES, KEY_WRITE, REG_PROCESS_APPKEY,
};

macro_rules! winapi_enum {
    ($t:ident, $doc:expr => [$($v:ident = $val:expr),*]) => {
        #[doc=$doc]
        #[allow(non_camel_case_types)]
        #[derive(Debug, PartialEq, Clone)]
        pub enum $t {
            $(
                #[doc = $doc]
                $v = $val,
            )*
        }
    };
}



winapi_enum!(NotifyFilter, "Enumeration of possible changes that should be reported in RegNotifyChangeKeyValue" => [
    REG_NOTIFY_CHANGE_NAME = 0x00000001,
    REG_NOTIFY_CHANGE_ATTRIBUTES=0x00000002,
    REG_NOTIFY_CHANGE_LAST_SET=0x00000004,
    REG_NOTIFY_CHANGE_SECURITY=0x00000008,
    REG_NOTIFY_THREAD_AGNOSTIC=0x10000000
]);

winapi_enum!(RegType, "Enumeration of possible registry value types" => [
REG_NONE=0x0,
REG_SZ=0x00000001,
REG_EXPAND_SZ=0x00000002,
REG_BINARY=0x00000003,
REG_DWORD=0x00000004,
REG_DWORD_BIG_ENDIAN=0x00000005,
REG_LINK=0x00000006,
REG_MULTI_SZ=0x00000007,
REG_RESOURCE_LIST=0x00000008,
REG_FULL_RESOURCE_DESCRIPTOR=0x00000009,
REG_RESOURCE_REQUIREMENTS_LIST=0x0000000A,
REG_QWORD=0x0000000B
]);
pub use self::RegType::*;

winapi_enum!(RegDisposition, "Enumeration of possible disposition values" => [
REG_CREATED_NEW_KEY=0x00000001,
REG_OPENED_EXISTING_KEY=0x00000002
]);
pub use self::RegDisposition::*;
