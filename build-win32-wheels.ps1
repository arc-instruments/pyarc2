param (
  [string]$interpreter,
  [switch]$debug
)

$patterns = @('*.pyd', '*.so')

foreach ($pat in $patterns) {
  $files = Get-ChildItem "pyarc2" -Filter $pat
  foreach ($f in $files) {
    echo "Clearing leftover $f"
    Remove-Item -Path $f.FullName
  }
}

if ($interpreter) {
  Write-Host "Trying to use interpreter $interpreter..." -nonewline
  if (-Not (Test-Path -Path $interpreter -PathType Leaf)) {
    echo " but interpreter $interpreter doesn't exist"
    Exit 1
  } else {
    if ( Invoke-Expression "& $interpreter -V" | Tee-Object -Variable pver ) {
      echo " found $pver"
    } else {
      echo " but could not determine the python interpreter version"
      Exit 1
    }
  }
} else {
  # try to locate default python
  Write-Host "Trying to use default python.exe interpreter..." -nonewline
  $application = Get-Command "python.exe"
  if( -Not $? ) {
    echo " but it could not be found; update PATH or specify and interpreter"
    Exit 1
  }
  $interpreter = $application.Source
  if ( Invoke-Expression "& $interpreter -V" | Tee-Object -Variable pver ) {
    echo " found $pver"
  } else {
    echo " but could not determine the python interpreter version"
    Exit 1
  }
}

python -m poetry install
python -m poetry update
if($debug) {
  python -m poetry run maturin build --release -v --interpreter $interpreter -F debug_packets
} else {
  python -m poetry run maturin build --release --interpreter $interpreter
}

if (-Not ($?)) {
  echo "Compilation failed"
  Exit 1
}

$wheels = Get-ChildItem "target\wheels" -Filter "*.whl"
foreach ($f in $wheels) {
  python -m poetry run delvewheel repair --add-path "." $f -w target\wheels
}
