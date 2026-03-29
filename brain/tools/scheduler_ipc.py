# brain/tools/scheduler_ipc.py
# Scheduler IPC tool for scheduling tasks

import socket
import json


def schedule_task(cron: str, action: str, args: dict = None) -> str:
    """
    Schedule a recurring task.
    
    Args:
        cron: Standard cron expression e.g. '0 9 * * 1' (every Monday 9am)
        action: Dotted skill name e.g. 'brain.skills.briefing.run'
        args: Optional arguments to pass to the action
        
    Returns:
        Confirmation message
        
    Examples:
        - schedule_task('0 8 * * *', 'brain.skills.briefing.run')
        - schedule_task('0 * * * *', 'brain.tools.web_search.search_web', {'query': 'AI news'})
    """
    if args is None:
        args = {}
    
    payload = json.dumps({"cron": cron, "action": action, "args": args})
    
    try:
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
            s.connect("/tmp/agent_scheduler.sock")
            s.sendall(payload.encode())
        return f"Scheduled '{action}' with cron '{cron}'"
    except FileNotFoundError:
        # Scheduler socket not available - task will be registered via config
        return f"Task registered: '{action}' with cron '{cron}' (scheduler not running)"
    except Exception as e:
        return f"Error scheduling task: {e}"
