@echo off
set GIT_DIR=D:\agent_sup\.git
set GIT_WORK_TREE=D:\agent_sup
cd /d D:\agent_sup

REM Resolve merge conflict by accepting our version
git checkout --ours README.md
git add README.md

REM Complete the merge commit
git commit -m "Merge remote main with local changes"

REM Push to main
git push origin master:main
