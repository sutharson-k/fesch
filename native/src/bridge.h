//! C++ Native Bridge Header
//! extern "C" API exposed to Rust FFI

#ifndef BRIDGE_H
#define BRIDGE_H

#ifdef __cplusplus
extern "C" {
#endif

// Mouse control
void move_mouse(int x, int y);
void click_mouse(int button);  // 0=left, 1=right, 2=middle

// Keyboard control
void type_text(const char* text);
void press_key(const char* key);  // "ctrl+c", "enter", "alt+f4"

// Screen control
void take_screenshot(const char* path);

// Window management
void focus_window(const char* title);
char* list_windows();  // Returns JSON string (caller must free)

// Process management
void launch_process(const char* cmd);
void kill_process(int pid);

#ifdef __cplusplus
}
#endif

#endif // BRIDGE_H
