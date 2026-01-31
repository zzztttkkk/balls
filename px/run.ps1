param(
    [ValidateSet("debug", "release")]
    [string]$mode = "release"
)

$pxdir = $PSScriptRoot
$proj_root = Split-Path -Path $pxdir -Parent
$deps = "$proj_root/target/$mode/deps"

& {
    $env:Path = "$deps;$env:Path"

    if ($IsWindows) {
        $env:Path = "$HOME/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin;$env:Path"
    }

    Start-Process "$proj_root/target/$mode/balls.exe" -WindowStyle Hidden 
}
