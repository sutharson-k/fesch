# brain/skills/README.md

# Skills

Skills are ADK sub-agents that extend the agent's capabilities.

## Creating a Skill

Create a new directory under `brain/skills/` with an `__init__.py` file:

```python
# brain/skills/my_skill/__init__.py
from google.adk.agents import Agent
from brain.model_router import OpenRouterModel

my_skill_agent = Agent(
    name="my_skill",
    model=OpenRouterModel(),
    description="My custom skill",
    instruction="Instructions for the skill",
    tools=[],
)

async def run(context: dict = None) -> str:
    """Entry point called by the scheduler."""
    ...
```

## Built-in Skills

- `briefing` - Morning news briefing
- `report_writer` - Weekly report generation
- `code_review` - Code review assistant
