@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars32.bat" >nul 2>&1
dumpbin /exports target\i686-pc-windows-msvc\release\war2modrust.dll | findstr w2p_init
