cargo build --release
copy appx\* target\release
cd target\release
powershell -command "Get-AppxPackage *0f8c5510-182f-4208-a48e-4215050a0453* | Remove-AppxPackage"
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
cd ..\..\