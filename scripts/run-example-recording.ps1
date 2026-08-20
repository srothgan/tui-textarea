param(
    [Parameter(Mandatory = $true, Position = 0)]
    [ValidateSet(
        "minimal",
        "editor",
        "single_line",
        "split",
        "variable",
        "vim",
        "popup_placeholder",
        "password",
        "wrap",
        "undo_coalescing"
    )]
    [string]$Example,

    [ValidatePattern('^\d+x\d+$')]
    [string]$Size = "120x30"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$previousRecording = [Environment]::GetEnvironmentVariable("TUI_TEXTAREA_RECORDING", "Process")
$previousSize = [Environment]::GetEnvironmentVariable("TUI_TEXTAREA_RECORDING_SIZE", "Process")

try {
    [Environment]::SetEnvironmentVariable("TUI_TEXTAREA_RECORDING", "1", "Process")
    [Environment]::SetEnvironmentVariable("TUI_TEXTAREA_RECORDING_SIZE", $Size, "Process")

    $cargoArguments = @("run", "--locked", "--example", $Example)
    if ($Example -eq "editor") {
        $fixtureDirectory = Join-Path $repositoryRoot "target\recording"
        $fixturePath = Join-Path $fixtureDirectory "editor.txt"
        [System.IO.Directory]::CreateDirectory($fixtureDirectory) | Out-Null
        [System.IO.File]::WriteAllLines($fixturePath, @(
            "Tui Textarea Recording",
            "",
            "Search this text with Ctrl+F.",
            "Edit the file, save it with Ctrl+S, and press Esc to exit."
        ))
        $cargoArguments += @("--features", "search", "--", $fixturePath)
    }

    Push-Location $repositoryRoot
    try {
        & cargo @cargoArguments
        if ($LASTEXITCODE -ne 0) {
            throw "The $Example example exited with code $LASTEXITCODE."
        }
    }
    finally {
        Pop-Location
    }
}
finally {
    [Environment]::SetEnvironmentVariable("TUI_TEXTAREA_RECORDING", $previousRecording, "Process")
    [Environment]::SetEnvironmentVariable("TUI_TEXTAREA_RECORDING_SIZE", $previousSize, "Process")
}
