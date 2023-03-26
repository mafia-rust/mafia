@echo off
for /l %%x in (1,1,16) do (
    start chrome http://localhost:3000/
)
