Add-Type -AssemblyName System.Drawing

$windowsImg = [System.Drawing.Image]::FromFile("$PSScriptRoot\windows.png")
Write-Host "windows.png: $($windowsImg.Width)x$($windowsImg.Height)"
$windowsImg.Dispose()

$macImg = [System.Drawing.Image]::FromFile("$PSScriptRoot\mac.png")
Write-Host "mac.png: $($macImg.Width)x$($macImg.Height)"
$macImg.Dispose()
