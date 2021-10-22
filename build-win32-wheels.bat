@echo off

python -m poetry install
python -m poetry update
python -m poetry run maturin build --release

IF %ERRORLEVEL% NEQ 0 (
  EXIT /B %ERRORLEVEL%
)

for /R %%a in (target\wheels\*.whl) DO python -m poetry run delvewheel repair --add-path %~dp0 %%a -w target\wheels\
