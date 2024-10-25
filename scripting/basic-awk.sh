# Sum up the fourth column of a csv file
awk -F',' 'NR==1{SUM=0; next} NR>1 {SUM += $4;} END {print SUM;}' file.csv

#Find the maximum number less than 150,000
awk 'NR==1{max = $1 + 0; next} {if ($1 > max && $1 < 150000) max = $1;} END {print max}'
