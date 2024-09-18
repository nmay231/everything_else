git diff --cached --name-status -r | awk '{ print $2 }' | awk -F / '{ print $1 ": " }'
