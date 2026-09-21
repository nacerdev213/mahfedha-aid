@echo off
chcp 65001 > nul
echo ===================================================
echo   جاري بناء واستخراج البرنامج بنسخة 32-بت (32-bit)...
echo ===================================================
echo.
call rustup target add i686-pc-windows-msvc
call npm run build:32
echo.
echo ===================================================
echo   اكتمل البناء! ستجد ملف exe داخل:
echo   src-tauri\target\i686-pc-windows-msvc\release\
echo ===================================================
pause
