Add-Type -AssemblyName System.Drawing

# Load the icon.png
$iconPath = "$PSScriptRoot\icon.png"
$icoPath = "$PSScriptRoot\icon.ico"

if (-not (Test-Path $iconPath)) {
    Write-Error "icon.png not found!"
    exit 1
}

# Load the PNG image
$img = [System.Drawing.Image]::FromFile($iconPath)

# Create icon from image
# ICO files need multiple sizes, but for simplicity we'll use common sizes
$sizes = @(16, 32, 48, 256)
$memoryStream = New-Object System.IO.MemoryStream
$writer = New-Object System.IO.BinaryWriter($memoryStream)

# ICO header
$writer.Write([UInt16]0)  # Reserved
$writer.Write([UInt16]1)  # Type (1 = ICO)
$writer.Write([UInt16]$sizes.Count)  # Number of images

$imageData = @()
$offset = 6 + ($sizes.Count * 16)  # Header + directory entries

foreach ($size in $sizes) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size)
    $graphics = [System.Drawing.Graphics]::FromImage($bmp)
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.DrawImage($img, 0, 0, $size, $size)

    # Save to PNG in memory
    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $data = $ms.ToArray()
    $imageData += ,@($data, $size)

    # Write directory entry (256 is represented as 0 in ICO format)
    $iconSize = if ($size -eq 256) { 0 } else { $size }
    $writer.Write([byte]$iconSize)  # Width
    $writer.Write([byte]$iconSize)  # Height
    $writer.Write([byte]0)  # Color palette
    $writer.Write([byte]0)  # Reserved
    $writer.Write([UInt16]1)  # Color planes
    $writer.Write([UInt16]32)  # Bits per pixel
    $writer.Write([UInt32]$data.Length)  # Size of image data
    $writer.Write([UInt32]$offset)  # Offset to image data

    $offset += $data.Length

    $graphics.Dispose()
    $bmp.Dispose()
    $ms.Dispose()
}

# Write image data
foreach ($entry in $imageData) {
    $writer.Write($entry[0])
}

# Save to file
[System.IO.File]::WriteAllBytes($icoPath, $memoryStream.ToArray())

$writer.Close()
$memoryStream.Close()
$img.Dispose()

Write-Host ".ico file created successfully!"
