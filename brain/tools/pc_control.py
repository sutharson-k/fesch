# brain/tools/pc_control.py
# PC control tools - wraps C++ native layer via ctypes

import ctypes
import os

# Load the native library
_lib_path = None
for path in [
    "./native/build/libagent_native.so",
    "./native/build/agent_native.dll",
    "../native/build/libagent_native.so",
    "../native/build/agent_native.dll",
]:
    if os.path.exists(path):
        _lib_path = path
        break

_lib = ctypes.CDLL(_lib_path) if _lib_path else None


def _check_lib():
    if _lib is None:
        raise RuntimeError("Native library not found. Build the C++ layer first.")


def move_mouse(x: int, y: int) -> str:
    """
    Move the mouse cursor to screen coordinates (x, y).

    Args:
        x: X coordinate (pixels from left)
        y: Y coordinate (pixels from top)

    Returns:
        Confirmation message
    """
    _check_lib()
    _lib.move_mouse(x, y)
    return f"Mouse moved to ({x}, {y})"


def click(button: int = 0) -> str:
    """
    Click the mouse.

    Args:
        button: 0=left, 1=right, 2=middle
        
    Returns:
        Confirmation message
    """
    _check_lib()
    _lib.click_mouse(button)
    buttons = ["left", "right", "middle"]
    return f"Clicked {buttons.get(button, 'left')} button"


def type_text(text: str) -> str:
    """
    Type text via keyboard simulation.

    Args:
        text: Text to type

    Returns:
        Confirmation message
    """
    _check_lib()
    _lib.type_text(text.encode())
    return f"Typed: {text}"


def press_key(combo: str) -> str:
    """
    Press a key combination e.g. 'ctrl+c', 'enter', 'alt+f4'.

    Args:
        combo: Key or key combination

    Returns:
        Confirmation message
    """
    _check_lib()
    _lib.press_key(combo.encode())
    return f"Pressed: {combo}"


def screenshot(path: str = "/tmp/screenshot.png") -> str:
    """
    Take a full-screen screenshot and save it to path.

    Args:
        path: Path to save the screenshot

    Returns:
        Confirmation message
    """
    _check_lib()
    _lib.take_screenshot(path.encode())
    return f"Screenshot saved to {path}"


def launch(cmd: str) -> str:
    """
    Launch an application or shell command.

    Args:
        cmd: Command or application to launch

    Returns:
        Confirmation message
    """
    _check_lib()
    _lib.launch_process(cmd.encode())
    return f"Launched: {cmd}"
