# brain/plugins/README.md

# Plugins

Plugins are callback hooks that run before/after model calls.

## Creating a Plugin

Create a new directory under `brain/plugins/` with an `__init__.py` file:

```python
# brain/plugins/my_plugin/__init__.py
from google.adk.agents.callback_context import CallbackContext
from google.adk.models import LlmRequest, LlmResponse

PLUGIN_META = {
    "name": "my_plugin",
    "hooks": ["before_model_call", "after_model_call"],
    "version": "1.0.0",
}

def before_model_call(ctx: CallbackContext, req: LlmRequest):
    """Fires before every LLM call."""
    print(f"[plugin] Request: {req.messages[-1]['content'][:80]}")

def after_model_call(ctx: CallbackContext, resp: LlmResponse):
    """Fires after every LLM response."""
    print(f"[plugin] Response: {str(resp)[:80]}")
```

## Built-in Plugins

- `logger` - Logs all requests and responses
- `memory_enhancer` - Enhances session memory
- `rate_limiter` - Rate limiting for API calls
