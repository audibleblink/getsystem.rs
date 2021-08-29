fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA,
        Windows::Win32::Security::{DuplicateTokenEx, ImpersonateLoggedOnUser, AdjustTokenPrivileges},
        Windows::Win32::System::Threading::{CreateProcessWithTokenW, OpenProcess, OpenProcessToken, GetCurrentProcess, GetCurrentProcessId, GetCurrentThread},
    );
}
