@echo off

python -m poetry run maturin build

IF %ERRORLEVEL% NEQ 0 (
  EXIT /B %ERRORLEVEL%
)

for /R %%a in (target\wheels\*.whl) DO python -m poetry run delvewheel repair %%a -w target\wheels\
