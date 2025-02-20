@echo off
REM Ensure script is run as administrator
net session >nul 2>&1
if not %errorLevel% == 0 (
    echo This script must be run as administrator.
    pause
    exit /b 1
)

REM Install Scoop if not already installed
where scoop >nul 2>&1
if %errorLevel% neq 0 (
    echo Installing Scoop...
    powershell -Command "Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force; iex (new-object net.webclient).downloadstring('https://get.scoop.sh')"
    set PATH=%PATH%;%USERPROFILE%\scoop\shims
)

REM Install necessary tools using Scoop
echo Installing dependencies...
scoop install git python llvm cmake curl

REM Install Python dependencies
echo Installing Python dependencies...
pip install mako

REM Build and run the project using Cargo
echo Building the project...
cargo run

echo Setup complete. Press any key to exit.
pause
