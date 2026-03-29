/**
 * PC Control - Native Windows/Mac/Linux PC control
 * Implements mouse, keyboard, window, and process management
 */

#include "bridge.h"
#include <cstdlib>
#include <cstring>
#include <cstdio>

#ifdef _WIN32
    #include <windows.h>
    #include <shellapi.h>
    #include <psapi.h>
#elif defined(__APPLE__)
    #include <ApplicationServices/ApplicationServices.h>
#else
    #include <X11/Xlib.h>
    #include <X11/keysym.h>
    #include <unistd.h>
#endif

// ============= Mouse Control =============

void move_mouse(int x, int y) {
#ifdef _WIN32
    SetCursorPos(x, y);
#elif defined(__APPLE__)
    CGEventRef event = CGEventCreateMouseEvent(nullptr, kCGEventMouseMoved, 
                                                CGPointMake(x, y), kCGMouseButtonLeft);
    CGEventPost(kCGHIDEventTap, event);
    CFRelease(event);
#else
    Display* display = XOpenDisplay(nullptr);
    XWarpPointer(display, None, DefaultRootWindow(display), 0, 0, 0, 0, x, y);
    XFlush(display);
    XCloseDisplay(display);
#endif
}

void click_mouse(int button) {
#ifdef _WIN32
    INPUT input = {0};
    input.type = INPUT_MOUSE;
    
    if (button == 0) {  // Left click
        input.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
        SendInput(1, &input, sizeof(INPUT));
        input.mi.dwFlags = MOUSEEVENTF_LEFTUP;
        SendInput(1, &input, sizeof(INPUT));
    } else if (button == 1) {  // Right click
        input.mi.dwFlags = MOUSEEVENTF_RIGHTDOWN;
        SendInput(1, &input, sizeof(INPUT));
        input.mi.dwFlags = MOUSEEVENTF_RIGHTUP;
        SendInput(1, &input, sizeof(INPUT));
    } else if (button == 2) {  // Middle click
        input.mi.dwFlags = MOUSEEVENTF_MIDDLEDOWN;
        SendInput(1, &input, sizeof(INPUT));
        input.mi.dwFlags = MOUSEEVENTF_MIDDLEUP;
        SendInput(1, &input, sizeof(INPUT));
    }
#elif defined(__APPLE__)
    CGMouseButton mb = kCGMouseButtonLeft;
    CGEventType down = kCGEventLeftMouseDown, up = kCGEventLeftMouseUp;
    
    if (button == 1) {
        mb = kCGMouseButtonRight;
        down = kCGEventRightMouseDown;
        up = kCGEventRightMouseUp;
    } else if (button == 2) {
        mb = kCGMouseButtonCenter;
        down = kCGEventOtherMouseDown;
        up = kCGEventOtherMouseUp;
    }
    
    CGPoint pos = CGEventGetLocation(CGEventCreate(nullptr));
    CGEventRef e1 = CGEventCreateMouseEvent(nullptr, down, pos, mb);
    CGEventPost(kCGHIDEventTap, e1);
    CFRelease(e1);
    
    CGEventRef e2 = CGEventCreateMouseEvent(nullptr, up, pos, mb);
    CGEventPost(kCGHIDEventTap, e2);
    CFRelease(e2);
#else
    Display* display = XOpenDisplay(nullptr);
    int button_num = Button1;
    if (button == 1) button_num = Button3;
    else if (button == 2) button_num = Button2;
    
    XTestFakeButtonEvent(display, button_num, True, CurrentTime);
    XTestFakeButtonEvent(display, button_num, False, CurrentTime);
    XFlush(display);
    XCloseDisplay(display);
#endif
}

// ============= Keyboard Control =============

void type_text(const char* text) {
#ifdef _WIN32
    for (const char* c = text; *c; ++c) {
        INPUT input = {0};
        input.type = INPUT_KEYBOARD;
        input.ki.wVk = VkKeyScanA(*c);
        SendInput(1, &input, sizeof(INPUT));
        
        input.ki.dwFlags = KEYEVENTF_KEYUP;
        SendInput(1, &input, sizeof(INPUT));
    }
#elif defined(__APPLE__)
    for (const char* c = text; *c; ++c) {
        CGEventRef keyDown = CGEventCreateKeyboardEvent(nullptr, 0, true);
        CGEventKeyboardSetUnicodeString(keyDown, 1, (UniChar*)c);
        CGEventPost(kCGHIDEventTap, keyDown);
        CFRelease(keyDown);
        
        CGEventRef keyUp = CGEventCreateKeyboardEvent(nullptr, 0, false);
        CGEventKeyboardSetUnicodeString(keyUp, 1, (UniChar*)c);
        CGEventPost(kCGHIDEventTap, keyUp);
        CFRelease(keyUp);
    }
#else
    Display* display = XOpenDisplay(nullptr);
    for (const char* c = text; *c; ++c) {
        KeySym keysym = XStringToKeysym(c);
        KeyCode keycode = XKeysymToKeycode(display, keysym);
        
        XTestFakeKeyEvent(display, keycode, True, CurrentTime);
        XTestFakeKeyEvent(display, keycode, False, CurrentTime);
    }
    XFlush(display);
    XCloseDisplay(display);
#endif
}

void press_key(const char* key) {
#ifdef _WIN32
    // Parse key combinations like "ctrl+c", "alt+f4", "enter"
    std::string key_str(key);
    
    struct KeyMapping {
        const char* name;
        WORD vk;
    };
    
    KeyMapping mappings[] = {
        {"enter", VK_RETURN}, {"ctrl", VK_CONTROL}, {"alt", VK_MENU},
        {"shift", VK_SHIFT}, {"win", VK_LWIN}, {"esc", VK_ESCAPE},
        {"tab", VK_TAB}, {"backspace", VK_BACK}, {"delete", VK_DELETE},
        {"up", VK_UP}, {"down", VK_DOWN}, {"left", VK_LEFT}, {"right", VK_RIGHT},
        {"f1", VK_F1}, {"f2", VK_F2}, {"f3", VK_F3}, {"f4", VK_F4},
        {"f5", VK_F5}, {"f6", VK_F6}, {"f7", VK_F7}, {"f8", VK_F8},
        {"f9", VK_F9}, {"f10", VK_F10}, {"f11", VK_F11}, {"f12", VK_F12},
        {nullptr, 0}
    };
    
    // Handle combinations
    if (key_str.find('+') != std::string::npos || key_str.find('-') != std::string::npos) {
        // Parse combination - simplified implementation
        char separator = key_str.find('+') != std::string::npos ? '+' : '-';
        size_t pos = key_str.find(separator);
        std::string modifier = key_str.substr(0, pos);
        std::string main_key = key_str.substr(pos + 1);
        
        // Press modifier
        for (int i = 0; mappings[i].name; ++i) {
            if (modifier == mappings[i].name) {
                keybd_event(mappings[i].vk, 0, 0, 0);
                break;
            }
        }
        
        // Press main key
        if (main_key.length() == 1) {
            keybd_event(VkKeyScanA(main_key[0]), 0, 0, 0);
            keybd_event(VkKeyScanA(main_key[0]), 0, KEYEVENTF_KEYUP, 0);
        }
        
        // Release modifier
        for (int i = 0; mappings[i].name; ++i) {
            if (modifier == mappings[i].name) {
                keybd_event(mappings[i].vk, 0, KEYEVENTF_KEYUP, 0);
                break;
            }
        }
    } else {
        // Single key
        for (int i = 0; mappings[i].name; ++i) {
            if (key_str == mappings[i].name) {
                keybd_event(mappings[i].vk, 0, 0, 0);
                keybd_event(mappings[i].vk, 0, KEYEVENTF_KEYUP, 0);
                break;
            }
        }
    }
#elif defined(__APPLE__)
    // macOS implementation
    // Simplified - full implementation would parse key names
#else
    // Linux implementation
    Display* display = XOpenDisplay(nullptr);
    // Simplified - full implementation would parse key names
    XCloseDisplay(display);
#endif
}

// ============= Screen Control =============

void take_screenshot(const char* path) {
#ifdef _WIN32
    // Get screen dimensions
    int width = GetSystemMetrics(SM_CXSCREEN);
    int height = GetSystemMetrics(SM_CYSCREEN);
    
    HDC hdcScreen = GetDC(nullptr);
    HDC hdcMem = CreateCompatibleDC(hdcScreen);
    HBITMAP hBitmap = CreateCompatibleBitmap(hdcScreen, width, height);
    
    SelectObject(hdcMem, hBitmap);
    BitBlt(hdcMem, 0, 0, width, height, hdcScreen, 0, 0, SRCCOPY);
    
    // Save as BMP (simplified - would need full BMP header writing)
    FILE* f = fopen(path, "wb");
    if (f) {
        // Write simple BMP header and data
        fclose(f);
    }
    
    DeleteDC(hdcMem);
    ReleaseDC(nullptr, hdcScreen);
    DeleteObject(hBitmap);
#elif defined(__APPLE__)
    // macOS screenshot using CGWindowListCreateImage
    CGDirectDisplayID displayID = CGMainDisplayID();
    CGImageRef image = CGDisplayCreateImage(displayID);
    // Save using CGImageDestination
#else
    // Linux - use scrot or similar
    char cmd[512];
    snprintf(cmd, sizeof(cmd), "scrot '%s'", path);
    system(cmd);
#endif
}

// ============= Window Management =============

void focus_window(const char* title) {
#ifdef _WIN32
    HWND hwnd = FindWindowA(nullptr, title);
    if (hwnd) {
        if (IsIconic(hwnd)) {
            ShowWindow(hwnd, SW_RESTORE);
        }
        SetForegroundWindow(hwnd);
    }
#elif defined(__APPLE__)
    // macOS window focusing
#else
    Display* display = XOpenDisplay(nullptr);
    // Linux window focusing
    XCloseDisplay(display);
#endif
}

char* list_windows() {
#ifdef _WIN32
    // Allocate buffer for window list (JSON format)
    static char buffer[4096] = "[";
    bool first = true;
    
    auto enum_proc = [](HWND hwnd, LPARAM lParam) -> BOOL {
        char* buf = (char*)lParam;
        char title[256];
        
        if (IsWindowVisible(hwnd) && GetWindowTextA(hwnd, title, sizeof(title)) > 0) {
            if (strlen(buf) > 1) strcat(buf, ",");
            strcat(buf, "{\"title\":\"");
            strcat(buf, title);
            strcat(buf, "\"}");
        }
        return TRUE;
    };
    
    EnumWindows(enum_proc, (LPARAM)buffer);
    strcat(buffer, "]");
    
    // Return a copy (caller must free)
    char* result = (char*)malloc(strlen(buffer) + 1);
    strcpy(result, buffer);
    return result;
#else
    // Linux/Mac implementation
    char* result = (char*)malloc(3);
    strcpy(result, "[]");
    return result;
#endif
}

// ============= Process Management =============

void launch_process(const char* cmd) {
#ifdef _WIN32
    ShellExecuteA(nullptr, "open", cmd, nullptr, nullptr, SW_SHOWNORMAL);
#else
    // Fork and exec on Unix-like systems
    pid_t pid = fork();
    if (pid == 0) {
        execl("/bin/sh", "sh", "-c", cmd, nullptr);
        exit(0);
    }
#endif
}

void kill_process(int pid) {
#ifdef _WIN32
    HANDLE hProcess = OpenProcess(PROCESS_TERMINATE, FALSE, pid);
    if (hProcess) {
        TerminateProcess(hProcess, 0);
        CloseHandle(hProcess);
    }
#else
    kill(pid, SIGTERM);
#endif
}
