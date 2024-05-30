find $DIR -type f -exec wc -l '{}'  ';' | awk '{x += $1} END {print x}'
