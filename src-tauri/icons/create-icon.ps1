Add-Type -AssemblyName System.Drawing

# Create a 512x512 bitmap
$bmp = New-Object System.Drawing.Bitmap(512, 512)
$graphics = [System.Drawing.Graphics]::FromImage($bmp)

# Fill with blue background
$graphics.Clear([System.Drawing.Color]::FromArgb(33, 150, 243))

# Draw F2F text
$font = New-Object System.Drawing.Font('Arial', 72, [System.Drawing.FontStyle]::Bold)
$brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$graphics.DrawString('F2F', $font, $brush, 150, 200)

# Save as PNG
$bmp.Save("$PSScriptRoot\icon.png", [System.Drawing.Imaging.ImageFormat]::Png)

# Create other sizes
$sizes = @(32, 128)
foreach ($size in $sizes) {
    $smallBmp = New-Object System.Drawing.Bitmap($size, $size)
    $smallGraphics = [System.Drawing.Graphics]::FromImage($smallBmp)
    $smallGraphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $smallGraphics.DrawImage($bmp, 0, 0, $size, $size)
    $smallBmp.Save("$PSScriptRoot\${size}x${size}.png", [System.Drawing.Imaging.ImageFormat]::Png)
    $smallGraphics.Dispose()
    $smallBmp.Dispose()
}

# Create 128x128@2x (which is actually 256x256)
$large = New-Object System.Drawing.Bitmap(256, 256)
$largeGraphics = [System.Drawing.Graphics]::FromImage($large)
$largeGraphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$largeGraphics.DrawImage($bmp, 0, 0, 256, 256)
$large.Save("$PSScriptRoot\128x128@2x.png", [System.Drawing.Imaging.ImageFormat]::Png)
$largeGraphics.Dispose()
$large.Dispose()

# Cleanup
$graphics.Dispose()
$bmp.Dispose()

Write-Host "Icon files created successfully!"
