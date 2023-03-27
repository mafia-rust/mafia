@echo off

set /a tabs=0
set SERVER_PORT=

REM Check if mafia_server is running on localhost:3000
powershell -Command "Test-NetConnection -ComputerName 'localhost' -Port 3000 -InformationLevel Quiet" > nul
if %ERRORLEVEL% EQU 0 (
  set SERVER_PORT=3000
)

REM Find port number of mafia_server.exe process using netstat
if not defined SERVER_PORT (
  for /f "tokens=1,2,3" %%a in ('netstat -ano ^| find "LISTENING" ^| find "127.0.0.1"') do (
      for /f "tokens=1,2" %%x in ('tasklist /FI "PID eq %%c" /FO CSV /NH') do (
          if "%%y" == "mafia_server.exe" (
              set SERVER_PORT=%%b
          )
      )
  )
)

if not defined SERVER_PORT (
    echo Mafia server is not running!
    exit /b 1
)

echo Mafia server is running on port %SERVER_PORT%
:loop
set /p choice=Enter choice (s=start, c=close, e=exit, n=new window): 

if /i "%choice:~0,1%"=="s" (
    set /a num=%choice:~1%
    if not defined num set num=1
    for /l %%x in (1,1,%num%) do (
        set /a tabs+=1
        start chrome http://localhost:%SERVER_PORT%/
    )
    goto loop
)

if /i "%choice:~0,1%"=="c" (
    if %tabs% gtr 0 (
        taskkill /im chrome.exe /fi "windowtitle eq http://localhost:%SERVER_PORT%/*"
        set /a tabs-=1
    )
    goto loop
)

if /i "%choice:~0,1%"=="n" (
    set /a num=%choice:~1%
    if not defined num set num=1
    set /a num-=1
    if %num% gtr 0 (
        start chrome http://localhost:%SERVER_PORT%/ --new-window
        for /l %%x in (1,1,%num%) do (
            start chrome http://localhost:%SERVER_PORT%/
        )
    ) else (
        start chrome http://localhost:%SERVER_PORT%/ --new-window
    )
    goto loop
)

if /i "%choice:~0,1%"=="e" (
    exit /b
)

goto loop
