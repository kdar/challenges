read expression
answer=$(echo "$expression" | bc -l)

printf "%.03f" "$answer"
