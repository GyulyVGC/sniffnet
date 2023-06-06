Write-Host "::group::WiX Toolset"
Invoke-WebRequest -Uri "https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip" -OutFile "$env:TEMP\\wix-binaries.zip" -Verbose
Expand-Archive -LiteralPath "$env:TEMP\\wix-binaries.zip" -DestinationPath "$env:TEMP\\wix" -Verbose
Set-Item -Path env:Path -Value "$env:Path;$env:TEMP\\wix"
Write-Host "::endgroup::"
