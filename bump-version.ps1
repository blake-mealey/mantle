param(
  [Parameter(Mandatory = $True)]
  [ValidateSet("Major","Minor","Patch")]
  [string]$VersionPart = "Patch"
)

function Write-Error($message) {
  [Console]::ForegroundColor = 'red'
  [Console]::Error.WriteLine($message)
  [Console]::ResetColor()
}

function Get-PackageVersion() {
  $PackageMetaRaw = cargo metadata --format-version 1
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to read crate meta data."
    return $Null
  }

  $PackageMeta = ConvertFrom-Json -InputObject $PackageMetaRaw
  return ($PackageMeta.packages | where { $_.name -eq "mantle" }).version
}

function Undo-Changes() {
  git checkout Cargo.lock Cargo.toml README.md > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to undo changes."
  }
}

function Undo-Tag($Tag) {
  git tag -d $Tag > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to undo tag."
  }
}

function Invoke-BumpVersion() {
  # Verify we are on the 'main' branch
  $BranchName = (git symbolic-ref --short HEAD).Trim()
  if ($BranchName -ne "main") {
    Write-Error "You can only bump the version from the 'main' branch. Switch with 'git checkout main'"
    return
  }

  # Verify there are no active changes
  $Changes = git status --porcelain
  if ($Changes -ne $Null) {
    Write-Error "You can only bump the version if there are no active changes. Stash or commit your changes to continue."
    return
  }

  # Verify build and tests are passing
  cargo test > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Build or test failed. Run 'cargo test' to see why."
    return
  }

  # Get the previous crate version
  $PreviousVersion = Get-PackageVersion
  if ($PreviousVersion -eq $Null) {
    return
  }

  # Install the cargo-bump tool
  cargo install cargo-bump > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to install 'cargo-bump' tool."
    return
  }

  # Bump the crate's patch version
  cargo bump $VersionPart.ToLower() > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to install 'cargo-bump' tool."
    Undo-Changes
    return
  }

  # Get the next crate version
  $NextVersion = Get-PackageVersion
  if ($NextVersion -eq $Null) {
    Undo-Changes
    return
  }

  Write-Host "📃 Bumping $($VersionPart): $PreviousVersion -> $NextVersion`n"

  # Update the README's install instructions to use the new version number
  $Readme = ((Get-Content -Path README.md -Raw) -replace "version = ""$PreviousVersion""","version = ""$NextVersion""").Trim()
  Set-Content -Path README.md $Readme

  # Stage changes to Git
  git add Cargo.lock Cargo.toml README.md > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to stage changes to Git."
    Undo-Changes
    return
  }

  # Commit changes to Git
  git commit -m "Bump version number ($NextVersion)" > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to commit changes to Git."
    Undo-Changes
    return
  }

  # Tag the commit with the current version
  $GitTag = "v$NextVersion"
  git tag $GitTag > $null 2> $null
  if ($LastExitCode -ne 0) {
    Write-Error "Failed to tag Git commit."
    Undo-Changes
    Undo-Tag $GitTag
    return
  }

  Write-Host "🎉 Bumped Mantle version to $NextVersion. Ready to push with 'git push origin && git push --tags'`n"

  $choices = "&Yes", "&No"
  $selectedIndex = $Host.UI.PromptForChoice($Null, "Automatically push changes to Git?", $choices, 0)

  if ($selectedIndex -eq 0) {
    git push origin && git push --tags > $null 2> $null
    if ($LastExitCode -ne 0) {
      Write-Error "Failed to push changes to Git."
      return
    }

    Write-Host "`n🚀 Changes pushed to Git!"
  }
}

Invoke-BumpVersion
