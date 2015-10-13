# for i in $(seq 1 50); do
#   echo $i
# done

x=1
while [ $x -le 50 ]; do
	echo $x
	x=$((x+1))
done
