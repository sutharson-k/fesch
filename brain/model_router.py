# brain/model_router.py
# Model configuration for OpenRouter

# Default model - using OpenRouter free model with 1M context
DEFAULT_MODEL = "google/gemini-2.0-flash-exp:free"

# Free large context models on OpenRouter
FREE_LARGE_CTX = [
    "google/gemini-2.0-flash-exp:free",  # 1M context
    "meta-llama/llama-3.3-70b-instruct:free",  # 128k context
    "deepseek/deepseek-r1:free",  # 164k context
]

# Model selection by task type (all via OpenRouter)
IMAGE_MODELS = [
    "black-forest-labs/flux-1.1-pro",
    "stabilityai/stable-diffusion-3-5-large",
]

VIDEO_MODELS = [
    "runway/gen3a_turbo",
    "minimax/video-01",
]

VISION_MODELS = [
    "google/gemini-2.0-flash-exp:free",
    "qwen/qwen2.5-vl-72b-instruct:free",
]


def select_model_for_task(task: str) -> str:
    """Select the appropriate OpenRouter model based on task description."""
    t = task.lower()

    # Image generation
    if any(w in t for w in ["draw", "generate image", "picture", "flux", "image"]):
        return f"openrouter/{IMAGE_MODELS[0]}"

    # Video generation
    if any(w in t for w in ["video", "animate", "render video", "runway"]):
        return f"openrouter/{VIDEO_MODELS[0]}"

    # Vision/image analysis
    if any(w in t for w in ["look at", "describe image", "what's in", "analyze image"]):
        return f"openrouter/{VISION_MODELS[0]}"

    # Default: free large context model
    return f"openrouter/{DEFAULT_MODEL}"


# Ollama configuration (offline models)
OLLAMA_BASE_URL = "http://localhost:11434"
OLLAMA_MODELS = ["llama3.2", "llama3.1", "mistral", "qwen2.5"]


def get_ollama_model_name(model: str = "llama3.2") -> str:
    """Get Ollama model identifier for litellm."""
    return f"ollama/{model}"


# Dashscope configuration (Alibaba Cloud)
DASHSCOPE_MODELS = ["qwen-max", "qwen-plus", "qwen-turbo"]


def get_dashscope_model_name(model: str = "qwen-max") -> str:
    """Get Dashscope model identifier for litellm."""
    return f"dashscope/{model}"
