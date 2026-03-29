@echo off
set GIT_DIR=D:\agent_sup\.git
set GIT_WORK_TREE=D:\agent_sup
cd /d D:\agent_sup
git add .
git commit -m "Fix ADK compatibility - use string model names and plain functions"
git push origin main
