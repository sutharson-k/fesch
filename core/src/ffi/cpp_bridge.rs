//! FFI bridge to C++ native layer for PC control

use anyhow::Result;
use std::ffi::CString;

#[link(name = "agent_native")]
extern "C" {
    fn move_mouse(x: i32, y: i32);
    fn click_mouse(button: i32);
    fn type_text(text: *const i8);
    fn press_key(key: *const i8);
    fn take_screenshot(path: *const i8);
    fn focus_window(title: *const i8);
    fn launch_process(cmd: *const i8);
    fn kill_process(pid: i32);
    fn list_windows() -> *mut i8;
}

/// Move the mouse to specified coordinates
pub fn move_mouse(x: i32, y: i32) -> Result<()> {
    unsafe {
        move_mouse(x, y);
    }
    Ok(())
}

/// Click the mouse (0=left, 1=right, 2=middle)
pub fn click_mouse(button: i32) -> Result<()> {
    unsafe {
        click_mouse(button);
    }
    Ok(())
}

/// Type text via keyboard simulation
pub fn type_text(text: &str) -> Result<()> {
    let c_str = CString::new(text)?;
    unsafe {
        type_text(c_str.as_ptr());
    }
    Ok(())
}

/// Press a key combination (e.g., "ctrl+c", "enter", "alt+f4")
pub fn press_key(key: &str) -> Result<()> {
    let c_str = CString::new(key)?;
    unsafe {
        press_key(c_str.as_ptr());
    }
    Ok(())
}

/// Take a screenshot and save to path
pub fn take_screenshot(path: &str) -> Result<()> {
    let c_str = CString::new(path)?;
    unsafe {
        take_screenshot(c_str.as_ptr());
    }
    Ok(())
}

/// Focus a window by title
pub fn focus_window(title: &str) -> Result<()> {
    let c_str = CString::new(title)?;
    unsafe {
        focus_window(c_str.as_ptr());
    }
    Ok(())
}

/// Launch a process or application
pub fn launch_process(cmd: &str) -> Result<()> {
    let c_str = CString::new(cmd)?;
    unsafe {
        launch_process(c_str.as_ptr());
    }
    Ok(())
}

/// Kill a process by PID
pub fn kill_process(pid: i32) -> Result<()> {
    unsafe {
        kill_process(pid);
    }
    Ok(())
}

/// List all open windows (returns JSON string)
pub fn list_windows() -> Result<String> {
    unsafe {
        let ptr = list_windows();
        if ptr.is_null() {
            return Ok("[]".to_string());
        }
        let c_str = std::ffi::CStr::from_ptr(ptr);
        let result = c_str.to_string_lossy().to_string();
        // Free the allocated memory (C++ side should provide free function)
        libc::free(ptr as *mut _);
        Ok(result)
    }
}
