# 🤖 FESCH AGENT

> **Polyglot AI Agent System**  
> **Stack:** Rust (core) · C++ (native bindings) · Python (AI/plugins)  
> **Theme:** Dracula · **Router:** OpenRouter · **Memory:** Google ADK

## Features

- 🖥️ **PC Control** - Mouse, keyboard, window, and process management via native C++
- 🌐 **Web Automation** - Search and browse using Playwright and Perplexity
- 🧠 **Google ADK** - Short-term memory and agent runtime
- 🔄 **Model Rotation** - OpenRouter with automatic model selection (image, video, vision)
- ⏰ **Task Scheduler** - Cron-based task scheduling
- 🔌 **Skills & Plugins** - Extensible via sub-agents and callback hooks
- 🎨 **Dracula TUI** - Beautiful terminal interface with ratatui
- 🏠 **Offline Support** - Ollama integration for local models
- 🌏 **Dashscope** - Alibaba Cloud model support

## Project Structure

```
agent_sup/
├── core/                    # Rust — orchestrator, scheduler, TUI
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # Entry point, TUI boot
│       ├── engine.rs        # Task queue, event loop (PyO3 → ADK)
│       ├── scheduler.rs     # Cron-style task scheduling
│       ├── memory.rs        # IPC bridge to ADK session memory
│       ├── model_router.rs  # Model rotation (OpenRouter)
│       ├── plugin_loader.rs # Loads /skills and /plugins
│       ├── tui/             # Dracula-themed TUI
│       └── ffi/             # FFI bindings to C++ layer
│
├── native/                  # C++ — PC control, screen, input
│   ├── CMakeLists.txt
│   └── src/
│       ├── bridge.h         # extern "C" API
│       └── pc_control.cpp   # Mouse, keyboard, windows, processes
│
├── brain/                   # Python — ADK agent brain
│   ├── __init__.py
│   ├── agent.py             # Root ADK Agent + Runner
│   ├── model_router.py      # OpenRouter/Ollama/Dashscope models
│   ├── tools/               # ADK @tool functions
│   ├── skills/              # ADK sub-agents
│   └── plugins/             # ADK callback hooks
│
├── config/
│   ├── models.toml          # Model rotation manifest
│   ├── schedule.toml        # Scheduled task definitions
│   └── agent.toml           # Global agent config
│
├── pyproject.toml
├── Makefile
└── agent.md                 # Full documentation
```

## Quick Start

### Prerequisites

- **Rust** (1.75+)
- **Python 3.12+** with `uv`
- **CMake** (for C++ build)
- **OpenRouter API Key** (free)

### Installation

```bash
# Clone and enter directory
cd agent_sup

# Install dependencies and build everything
make install

# Or build step by step
make native    # Build C++ layer
make core      # Build Rust core
make brain     # Install Python dependencies
```

### Configuration

1. Set your API keys in `brain/.env`:

```bash
OPENROUTER_API_KEY=sk-or-your-key-here
```

2. (Optional) Configure offline models in `config/models.toml`

### Running

```bash
# Run the TUI
make run

# Or run CLI mode
make run-cli

# Or start ADK web UI
make adk-web
```

## Tools

| Tool | Description |
|------|-------------|
| `search_web` | Search using Perplexity via OpenRouter |
| `browse` | Browser automation with Playwright |
| `move_mouse` | Move mouse to coordinates |
| `click` | Click mouse (left/right/middle) |
| `type_text` | Type text via keyboard |
| `press_key` | Press key combinations (ctrl+c, alt+f4) |
| `screenshot` | Take screenshots |
| `launch` | Launch applications |
| `read_file` | Read file contents |
| `write_file` | Write to files |
| `list_dir` | List directory contents |
| `schedule_task` | Schedule recurring tasks |

## Model Rotation

The agent automatically selects the best model based on task type:

| Task Type | Model |
|-----------|-------|
| General chat | `gemini-2.0-flash-exp:free` (1M context) |
| Image generation | `flux-1.1-pro` |
| Video generation | `runway/gen3a_turbo` |
| Vision/analysis | `gemini-2.0-flash-exp:free` |
| Offline | `ollama/llama3.2` |

## Creating Skills

```python
# brain/skills/my_skill/__init__.py
from google.adk.agents import Agent
from brain.model_router import OpenRouterModel

my_skill_agent = Agent(
    name="my_skill",
    model=OpenRouterModel(),
    description="My custom skill",
    instruction="What this skill does",
    tools=[],
)
```

## Creating Plugins

```python
# brain/plugins/logger/__init__.py
from google.adk.agents.callback_context import CallbackContext

def before_model_call(ctx: CallbackContext, req):
    print(f"Request: {req.messages[-1]['content'][:80]}")

def after_model_call(ctx: CallbackContext, resp):
    print(f"Response: {str(resp)[:80]}")
```

## Dracula Theme

| Color | Hex | Usage |
|-------|-----|-------|
| Background | `#282a36` | Main background |
| Surface | `#44475a` | Panels, input |
| Comment | `#6272a4` | Secondary text |
| Pink | `#ff79c6` | Agent responses |
| Purple | `#bd93f9` | Headers |
| Cyan | `#8be9fd` | User input |
| Green | `#50fa7b` | Success |
| Orange | `#ffb86c` | Warnings |
| Foreground | `#f8f8f2` | Body text |

## License

MIT
