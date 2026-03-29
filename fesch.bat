@echo off
REM Fesch Agent CLI Launcher

setlocal
set "PATH=%USERPROFILE%\.local\bin;%PATH%"

if "%~1"=="" goto :cli
if /i "%~1"=="run" goto :run
if /i "%~1"=="web" goto :web
if /i "%~1"=="cli" goto :cli
if /i "%~1"=="build" goto :build
if /i "%~1"=="clean" goto :clean
if /i "%~1"=="help" goto :help

REM Default: treat as message for CLI
goto :cli

:run
echo 🚀 Running Fesch Agent TUI...
cd /d D:\agent_sup\core
cargo run --release
goto :end

:web
echo 🌐 Starting Fesch Agent Web UI...
cd /d D:\agent_sup
uv run adk web brain/
goto :end

:cli
echo 💬 Running Fesch Agent CLI...
echo.
cd /d D:\agent_sup
uv run adk run brain/
goto :end

:build
echo 🔨 Building Fesch Agent...
cd /d D:\agent_sup
uv sync
cd core
cargo build --release
goto :end

:clean
echo 🧹 Cleaning Fesch Agent...
cd /d D:\agent_sup
rmdir /s /q core\target 2>nul
rmdir /s /q native\build 2>nul
rmdir /s /q brain\__pycache__ 2>nul
rmdir /s /q .venv 2>nul
goto :end

:help
echo.
echo 🤖 Fesch Agent - Polyglot AI Agent System
echo.
echo Usage:
echo   fesch       - Run CLI mode (default)
echo   fesch run   - Run the TUI interface
echo   fesch web   - Start the ADK web UI
echo   fesch cli   - Run a CLI command
echo   fesch build - Build all components
echo   fesch clean - Clean build artifacts
echo   fesch help  - Show this help
echo.

:end
endlocal
