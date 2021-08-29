use bindings::Windows::Win32::Security::{
    DuplicateTokenEx, ImpersonateLoggedOnUser, AdjustTokenPrivileges,
    TOKEN_READ_CONTROL, TOKEN_IMPERSONATE, TOKEN_DUPLICATE,
};
use bindings::Windows::Win32::System::Threading::{
    CreateProcessWithTokenW, OpenProcess, OpenProcessToken,
    GetCurrentProcess, GetCurrentProcessId, GetCurrentThread,
    PROCESS_QUERY_INFORMATION,
};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
use bindings::Windows::Win32::Foundation::HANDLE;

fn main() {
    unsafe {
        MessageBoxA(None, "BEFORE", "World", MB_OK);
        let tokenH = get_token_from_pid(1156);
        MessageBoxA(None, "MIDDLE", "World", MB_OK);
        ImpersonateLoggedOnUser(tokenH);
        MessageBoxA(None, "AFTER", "World", MB_OK);
    }
}

fn get_token_from_pid(pid: u32) -> HANDLE {
     unsafe {
     let hProc =   OpenProcess(PROCESS_QUERY_INFORMATION, false, pid);

     let newHToken: HANDLE = HANDLE(0);

    let ok = OpenProcessToken(
        hProc,
        TOKEN_READ_CONTROL | TOKEN_IMPERSONATE | TOKEN_DUPLICATE ,
        &newHToken as *const HANDLE as _, 
        ).as_bool();

    return newHToken;
    }
}
