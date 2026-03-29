# brain/tools/file_system.py
# File system tools for reading, writing, and listing files

import os
from pathlib import Path


def read_file(path: str) -> str:
    """
    Read the contents of a file.
    
    Args:
        path: Path to the file to read
        
    Returns:
        File contents as string
    """
    try:
        p = Path(path).expanduser().resolve()
        return p.read_text(encoding="utf-8")
    except Exception as e:
        return f"Error reading file: {e}"


def write_file(path: str, content: str) -> str:
    """
    Write content to a file.

    Args:
        path: Path to the file to write
        content: Content to write

    Returns:
        Confirmation message
    """
    try:
        p = Path(path).expanduser().resolve()
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(content, encoding="utf-8")
        return f"Successfully wrote to {path}"
    except Exception as e:
        return f"Error writing file: {e}"


def list_dir(path: str = ".") -> str:
    """
    List contents of a directory.
    
    Args:
        path: Path to the directory (default: current directory)
        
    Returns:
        Directory listing as string
    """
    try:
        p = Path(path).expanduser().resolve()
        if not p.is_dir():
            return f"Error: {path} is not a directory"
        
        entries = []
        for entry in p.iterdir():
            entry_type = "📁" if entry.is_dir() else "📄"
            entries.append(f"{entry_type} {entry.name}")
        
        return "\n".join(sorted(entries))
    except Exception as e:
        return f"Error listing directory: {e}"
