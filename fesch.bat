@echo off
REM Fesch Agent CLI Launcher

setlocal
set "PATH=%USERPROFILE%\.local\bin;%PATH%"

if "%1"=="" goto :cli
if "%1"=="run" goto :run
if "%1"=="web" goto :web
if "%1"=="cli" goto :cli
if "%1"=="build" goto :build
if "%1"=="clean" goto :clean
if "%1"=="help" goto :help

echo Unknown command: %1
goto :help

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
cd /d D:\agent_sup
if "%2"=="" (
    uv run adk run brain/
) else (
    uv run adk run brain/ --message "%2"
)
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
