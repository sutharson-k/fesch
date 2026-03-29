# brain/tools/web_automation.py
# Browser automation using Playwright

import asyncio
from playwright.async_api import async_playwright


async def browse(task: str) -> str:
    """
    Automate a real browser to complete a web task in natural language.
    
    Examples:
        - "go to gmail and check inbox"
        - "buy headphones on amazon"
        - "search for python tutorials on youtube"
    
    Args:
        task: Natural language description of the web task
        
    Returns:
        Result of the browsing automation
    """
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=False)
        page = await browser.new_page()
        result = await _agentic_browser_loop(page, task)
        await browser.close()
        return result


async def _agentic_browser_loop(page, task: str) -> str:
    """
    Agentic browser loop - screenshot → LLM decides → execute → repeat.
    
    This is a simplified implementation. A full implementation would:
    1. Take screenshot
    2. Send to LLM with task description
    3. LLM returns next action (click, type, navigate, etc.)
    4. Execute action
    5. Repeat until task is complete
    """
    # For now, just navigate and return page title
    # Full implementation would use an LLM to decide actions
    
    # Extract URL from task if present
    urls = extract_urls(task)
    if urls:
        await page.goto(urls[0])
        await page.wait_for_load_state("networkidle")
        return f"Navigated to {urls[0]}. Page title: {await page.title()}"
    
    # Search query
    await page.goto("https://www.google.com")
    search_box = await page.query_selector('input[name="q"]')
    if search_box:
        await search_box.fill(task)
        await page.keyboard.press("Enter")
        await page.wait_for_load_state("networkidle")
        return f"Searched for: {task}. Page title: {await page.title()}"
    
    return "Unable to complete browsing task"


def extract_urls(text: str) -> list:
    """Extract URLs from text."""
    import re
    url_pattern = r'https?://[^\s<>"{}|\\^`\[\]]+'
    return re.findall(url_pattern, text)
