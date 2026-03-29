# brain/tools/__init__.py
# Tool registry

from brain.tools.web_search import search_web
from brain.tools.web_automation import browse
from brain.tools.pc_control import (
    move_mouse,
    click,
    type_text,
    press_key,
    screenshot,
    launch,
)
from brain.tools.file_system import read_file, write_file, list_dir
from brain.tools.scheduler_ipc import schedule_task

TOOL_REGISTRY = [
    search_web,
    browse,
    move_mouse,
    click,
    type_text,
    press_key,
    screenshot,
    launch,
    read_file,
    write_file,
    list_dir,
    schedule_task,
]

__all__ = [
    "search_web",
    "browse",
    "move_mouse",
    "click",
    "type_text",
    "press_key",
    "screenshot",
    "launch",
    "read_file",
    "write_file",
    "list_dir",
    "schedule_task",
    "TOOL_REGISTRY",
]
