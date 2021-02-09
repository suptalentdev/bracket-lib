@ECHO OFF
cargo build --all --target wasm32-unknown-unknown --release --features=opengl

REM Build the actual WASM files and helpers
CALL :StageExample alpha, alpha
CALL :StageExample astar_mouse, astar_mouse
CALL :StageExample benchmark, benchmark
CALL :StageExample colorfont, colorfont
CALL :StageExample dwarfmap, dwarfmap
CALL :StageExample flexible, flexible
CALL :StageExample fontswitch, fontswitch
CALL :StageExample hello_minimal, hello_minimal
CALL :StageExample hello_terminal, hello_terminal
CALL :StageExample input_harness, input_harness
CALL :StageExample keyboard, keyboard
CALL :StageExample native_gl, native_gl
CALL :StageExample no_cls, no_cls
CALL :StageExample postprocess, postprocess
CALL :StageExample rex, rex
CALL :StageExample sparse, sparse
CALL :StageExample sprites, sprites
CALL :StageExample textblock, textblock
CALL :StageExample textsprites, textsprites
CALL :StageExample tiles, tiles
CALL :StageExample unicode, unicode
CALL :StageExample virtual_console, virtual_console
CALL :StageExample walking, walking


REM Submit to server
cd wasm_help\staging
pscp -r * herbert@vps.bracketproductions.com:/var/www/bfnightly/wasmtest
cd ..\..

REM Finish
EXIT /B 0

REM Usage: StageExample EXAMPLE
:StageExample
echo Building example %~1
cargo build --example %~1 --target wasm32-unknown-unknown --release --features=opengl
echo wasm-gc ..\target\wasm32-unknown-unknown\release\examples\%~1.wasm
mkdir .\wasm_help\staging\%~2
wasm-bindgen ..\target\wasm32-unknown-unknown\release\examples\%~1.wasm --out-dir .\wasm_help\staging\%~2 --no-modules --no-typescript
copy .\wasm_help\index.html .\wasm_help\staging\%~2
move .\wasm_help\staging\%~2\%~1_bg.wasm .\wasm_help\staging\%~2\myblob_bg.wasm
move .\wasm_help\staging\%~2\%~1.js .\wasm_help\staging\%~2\myblob.js
EXIT /B 0
