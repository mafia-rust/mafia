@echo off

set /a tabs=0

:loop
set /p choice=Enter choice (a=add tab, c=close, e=exit, n=new window, s=start game): 

if /i "%choice:~0,1%"=="a" (
    set /a num=%choice:~1%
    if not defined num set num=1
    for /l %%x in (1,1,%num%) do (
        set /a tabs+=1
        start chrome http://localhost:3000/
    )
    goto loop
)

if /i "%choice:~0,1%"=="c" (
    if %tabs% gtr 0 (
        taskkill /im chrome.exe /fi "windowtitle eq http://localhost:3000/*"
        set /a tabs-=1
    )
    goto loop
)

if /i "%choice:~0,1%"=="n" (
    set /a num=%choice:~1%
    if not defined num set num=1
    set /a num-=1
    if %num% gtr 0 (
        start chrome http://localhost:3000/ --new-window
        for /l %%x in (1,1,%num%) do (
            start chrome http://localhost:3000/
        )
    ) else (
        start chrome http://localhost:3000/ --new-window
    )
    goto loop
)

if /i "%choice:~0,1%"=="e" (
    exit /b
)

goto loop
