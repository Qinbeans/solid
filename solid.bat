echo off
set arg1=%1

if "%arg1%"=="run" (
    echo Running Solid
    cargo run
) else (
    if "%arg1%"=="build" (
        echo Building Solid
        cargo build
    ) else (
        echo Invalid argument
    )
)