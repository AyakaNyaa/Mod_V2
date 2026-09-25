@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars32.bat" >nul
cargo build --release --target i686-pc-windows-msvc %*
copy /y target\i686-pc-windows-msvc\release\war2modrust.dll victory.w2p
