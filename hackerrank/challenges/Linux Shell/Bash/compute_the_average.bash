read N
sum=0
for i in $(seq 1 $N); do
  read x
  sum=$(($sum + $x))
done

answer=$(echo "$sum / $N" | bc -l)

printf "%.03f" "$answer"
