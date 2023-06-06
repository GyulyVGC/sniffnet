[CmdletBinding()]
param (
	[Parameter(Mandatory)]
	[ValidateSet("amd64", "i386", "arm64")]
	[string]$ARCH,
	[Parameter(Mandatory)]
	[string]$OEM
)

Write-Host "::group::Npcap SDK"
Invoke-WebRequest -Uri "https://npcap.com/dist/npcap-sdk-1.13.zip" -OutFile "$env:TEMP\\npcap-sdk.zip" -Verbose
Expand-Archive -LiteralPath "$env:TEMP\\npcap-sdk.zip" -DestinationPath "$env:TEMP\\npcap-sdk" -Verbose
$LibPath = switch ($ARCH)
{
	"amd64" { "Lib\\x64"   }
	"i386"  { "Lib"        }
	"arm64" { "Lib\\ARM64" }
}
Add-Content -Path "$env:GITHUB_ENV" -Value "LIB=$env:TEMP\\npcap-sdk\\$LibPath"
Write-Host "::endgroup::"

Write-Host "::group::Npcap DLL"
Invoke-WebRequest -Uri "$OEM" -OutFile "$env:TEMP\\npcap-oem.exe" -Verbose
Start-Process -FilePath "$env:TEMP\\npcap-oem.exe" -ArgumentList "/S" -Wait
Write-Host "::endgroup::"
