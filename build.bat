@echo off
setlocal

if not exist "C:\msys64" (
    echo Installing MSYS2...
    winget install -e --id MSYS2.MSYS2
)

echo Installing g++ via pacman...
C:\msys64\usr\bin\bash -lc "pacman -S --noconfirm --needed mingw-w64-x86_64-gcc"

set PATH=C:\msys64\mingw64\bin;%PATH%

rustup show | findstr "x86_64-pc-windows-gnu" >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Rust GNU toolchain...
    rustup install stable-x86_64-pc-windows-gnu
    rustup default stable-x86_64-pc-windows-gnu
)

echo Compiling prime.cpp...
g++ -c prime.cpp -o prime.o

echo Building Rust project...
cargo rustc --release -- -Clink-arg=prime.o -Clink-arg=-lkernel32


endlocal
