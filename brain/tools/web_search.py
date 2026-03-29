# brain/tools/web_search.py
# Web search tool using OpenRouter Perplexity

import os
import httpx
from google.adk.tools import tool


@tool
def search_web(query: str) -> str:
    """
    Search the web for current information using Perplexity sonar-pro via OpenRouter.
    
    Args:
        query: The search query
        
    Returns:
        Search results as text
    """
    api_key = os.getenv("OPENROUTER_API_KEY", "")
    
    resp = httpx.post(
        "https://openrouter.ai/api/v1/chat/completions",
        headers={
            "Authorization": f"Bearer {api_key}",
            "HTTP-Referer": "https://github.com/fesch-agent",
            "X-Title": "Fesch Agent",
        },
        json={
            "model": "perplexity/sonar-pro",
            "messages": [{"role": "user", "content": query}],
        },
        timeout=30,
    )
    resp.raise_for_status()
    return resp.json()["choices"][0]["message"]["content"]
