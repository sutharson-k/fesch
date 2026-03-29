# brain/model_router.py
# OpenRouter model wrapper with rotation support

import os
import httpx
from google.adk.models.base_llm import BaseLlm

FREE_LARGE_CTX = [
    "google/gemini-2.0-flash-exp:free",  # 1M context — default
    "meta-llama/llama-3.3-70b-instruct:free",  # 128k context
    "deepseek/deepseek-r1:free",  # 164k context
]


class OpenRouterModel(BaseLlm):
    """Drop-in ADK model that routes all LLM calls through OpenRouter."""

    def __init__(self, model: str = FREE_LARGE_CTX[0]):
        self.model = model
        self.api_key = os.getenv("OPENROUTER_API_KEY", "")
        self.base = "https://openrouter.ai/api/v1"

    @property
    def model_name(self) -> str:
        return self.model

    def _call(self, messages: list, **kwargs) -> str:
        resp = httpx.post(
            f"{self.base}/chat/completions",
            headers={
                "Authorization": f"Bearer {self.api_key}",
                "HTTP-Referer": "https://github.com/fesch-agent",
                "X-Title": "Fesch Agent",
            },
            json={"model": self.model, "messages": messages, **kwargs},
            timeout=120,
        )
        resp.raise_for_status()
        return resp.json()["choices"][0]["message"]["content"]

    def select_for_task(self, task: str) -> "OpenRouterModel":
        """Return the right model for the detected task type."""
        t = task.lower()

        # Image generation
        if any(
            w in t for w in ["draw", "generate image", "picture", "flux", "image"]
        ):
            return OpenRouterModel("black-forest-labs/flux-1.1-pro")

        # Video generation
        if any(
            w in t for w in ["video", "animate", "render video", "runway"]
        ):
            return OpenRouterModel("runway/gen3a_turbo")

        # Vision/image analysis
        if any(
            w in t for w in ["look at", "describe image", "what's in", "analyze image"]
        ):
            return OpenRouterModel("google/gemini-2.0-flash-exp:free")

        # Default: free large context model
        return OpenRouterModel(FREE_LARGE_CTX[0])


class OllamaModel(BaseLlm):
    """Offline model support via Ollama."""

    def __init__(self, model: str = "llama3.2", base_url: str = "http://localhost:11434"):
        self.model = model
        self.base_url = base_url

    @property
    def model_name(self) -> str:
        return f"ollama/{self.model}"

    def _call(self, messages: list, **kwargs) -> str:
        resp = httpx.post(
            f"{self.base_url}/api/chat",
            json={
                "model": self.model,
                "messages": messages,
                "stream": False,
            },
            timeout=120,
        )
        resp.raise_for_status()
        return resp.json()["message"]["content"]


class DashscopeModel(BaseLlm):
    """Dashscope (Alibaba Cloud) model support."""

    def __init__(self, model: str = "qwen-max"):
        self.model = model
        self.api_key = os.getenv("DASHSCOPE_API_KEY", "")
        self.base = "https://dashscope.aliyuncs.com/api/v1"

    @property
    def model_name(self) -> str:
        return f"dashscope/{self.model}"

    def _call(self, messages: list, **kwargs) -> str:
        # Convert messages to Dashscope format
        dashscope_messages = [{"role": m["role"], "content": m["content"]} for m in messages]

        resp = httpx.post(
            f"{self.base}/services/aigc/text-generation/generation",
            headers={
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json",
            },
            json={
                "model": self.model,
                "input": {"messages": dashscope_messages},
            },
            timeout=120,
        )
        resp.raise_for_status()
        result = resp.json()
        return result["output"]["choices"][0]["message"]["content"]
