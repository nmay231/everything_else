# NOTE: Originally written by someone else, but formatted and
# expanded upon by AI because I could not be bothered...


# Define paths
$directoryPath = "directoryhere"
$logFile = "rename-log.txt"

# Ensure log file exists
if (-not (Test-Path $logFile)) {
    New-Item -ItemType File -Path $logFile | Out-Null
}

# Get all .mdb files recursively
$mdbFiles = Get-ChildItem -Path $directoryPath -Filter "*.mdb" -Recurse -File

foreach ($file in $mdbFiles) {
    $oldPath = $file.FullName
    $newName = "$($file.Name).old"
    $newPath = Join-Path $file.DirectoryName $newName

    # Skip if target already exists
    if (-not (Test-Path $newPath)) {
        Rename-Item -Path $oldPath -NewName $newName

        # Log the rename
        "{0} => {1}" -f $oldPath, $newPath | Out-File -FilePath $logFile -Append
    }
}
