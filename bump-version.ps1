function Get-PackageVersion() {
  $PackageMeta = ConvertFrom-Json -InputObject $(cargo metadata --format-version 1)
  return ($PackageMeta.packages | where { $_.name -eq "rocat" }).version
}

$PreviousVersion = Get-PackageVersion

cargo install cargo-bump
cargo bump patch

$NextVersion = Get-PackageVersion

$Readme = ((Get-Content -Path README.md -Raw) -replace "version = ""$PreviousVersion""","version = ""$NextVersion""").Trim()
Set-Content -Path README.md $Readme

git tag $NextVersion
