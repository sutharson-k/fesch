# brain/agent.py
# Real ADK agent definition

import os
import importlib
import pkgutil
from dotenv import load_dotenv

from google.adk.agents import Agent
from google.adk.runners import Runner
from google.adk.sessions import InMemorySessionService

from brain.model_router import DEFAULT_MODEL, select_model_for_task
from brain.tools.web_search import search_web
from brain.tools.web_automation import browse
from brain.tools.pc_control import (
    move_mouse, click, type_text, press_key,
    screenshot, launch
)
from brain.tools.file_system import read_file, write_file, list_dir
from brain.tools.scheduler_ipc import schedule_task

load_dotenv()

# Get API key for OpenRouter
OPENROUTER_API_KEY = os.getenv("OPENROUTER_API_KEY", "")

# ── Root ADK Agent ───────────────────────────────────────────────
root_agent = Agent(
    name="fesch_agent",
    model=DEFAULT_MODEL,  # Using string model name directly
    description=(
        "Fesch - General-purpose AI agent with PC control, "
        "web browsing, scheduling, and plugin support."
    ),
    instruction=(
        "You are Fesch, a powerful AI assistant. You can control the user's PC, "
        "browse the web, search for information, manage files, and schedule "
        "tasks. Think step-by-step. Use tools whenever possible. "
        "Be concise and precise."
    ),
    tools=[
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
    ],
)

# ── ADK Session Memory (short-term, in-process) ──────────────────
_session_service = InMemorySessionService()

runner = Runner(
    agent=root_agent,
    app_name="fesch_agent",
    session_service=_session_service,
)

# ── Auto-load Skills (ADK sub-agents) ────────────────────────────
for _, name, _ in pkgutil.iter_modules(["brain/skills"]):
    mod = importlib.import_module(f"brain.skills.{name}")
    if hasattr(mod, f"{name}_agent"):
        root_agent.tools.append(getattr(mod, f"{name}_agent"))

# ── Auto-load Plugins (ADK callback hooks) ───────────────────────
for _, name, _ in pkgutil.iter_modules(["brain/plugins"]):
    mod = importlib.import_module(f"brain.plugins.{name}")
    if hasattr(mod, "before_model_call"):
        root_agent.before_model_callback = mod.before_model_call
    if hasattr(mod, "after_model_call"):
        root_agent.after_model_callback = mod.after_model_call


# ── Chat Function Called by Rust (via PyO3) ──────────────────────
import asyncio
from google.genai import types as genai_types


async def chat(user_id: str, session_id: str, message: str) -> str:
    """Called by the Rust TUI engine for each user turn."""
    # Swap model based on task type before running
    root_agent.model = select_model_for_task(message)

    content = genai_types.Content(
        role="user",
        parts=[genai_types.Part(text=message)],
    )
    response = ""
    async for event in runner.run_async(
        user_id=user_id,
        session_id=session_id,
        new_message=content,
    ):
        if event.is_final_response():
            response = event.response.parts[0].text
    return response


def chat_sync(user_id: str, session_id: str, message: str) -> str:
    """Synchronous wrapper for PyO3 / Rust calls."""
    return asyncio.run(chat(user_id, session_id, message))
