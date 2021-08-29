use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
use bindings::Windows::Win32::Security::{DuplicateTokenEx, ImpersonateLoggedOnUser, AdjustTokenPrivileges};
use bindings::Windows::Win32::System::Threading::{CreateProcessWithTokenW, OpenProcess, OpenProcessToken, GetCurrentProcess, GetCurrentProcessId, GetCurrentThread};

fn main() {
    unsafe {
        MessageBoxA(None, "Hello", "World", MB_OK);
    }
}
