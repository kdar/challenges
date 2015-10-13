#!/bin/bash

# The recursive way would be the natural way to write this (and shorter).
# Decided to do the harder/faster iterative way.

ISTEST="$1"
TEST=$(echo -e "5\n")
ROWS=63
COLS=100
LEN=16

doread() {
  local  __resultvar=$1;

  line="";
  if [ "$ISTEST" = "test" ]; then
    if [[ ${#TEST} -eq 0 ]]; then
      return 1
    fi
    read line <<< "$TEST";
    TEST=${TEST:${#line}+1};
  else
    read line;
  fi

  status=$?

  if [[ "$__resultvar" ]]; then
    eval $__resultvar="'$line'";
  else
    echo "$line";
  fi

  return $status
}

solve() {
  doread N;

  declare -A matrix

  for (( i=1; i <= ROWS; i++ )) do
    for (( j=1; j <= COLS; j++ )) do
      matrix[$i,$j]="_"
    done
  done

  START_Y=$ROWS
  START_X=($((COLS/2)))
  START_LEN=$LEN

  for (( n=0; n < N; n++ )) do
    # draw stem
    for (( i=START_Y; i > (START_Y-START_LEN); i-- )) do
      for (( j=0; j < ${#START_X[@]}; j++ )) do
        matrix[$i,${START_X[$j]}]="1"
      done
    done

    # draw branches
    START_Y=$((START_Y-START_LEN))
    BRANCH_OFFSET=1
    for (( i=START_Y; i > (START_Y-START_LEN); i-- )) do
      for (( j=0; j < ${#START_X[@]}; j++ )) do
        matrix[$i,$((${START_X[$j]}-BRANCH_OFFSET))]="1"
        matrix[$i,$((${START_X[$j]}+BRANCH_OFFSET))]="1"
      done
      BRANCH_OFFSET=$((BRANCH_OFFSET+1))
    done

    # calculate new variables
    START_Y=$((START_Y-START_LEN))
    NEW_START_X=()
    for (( j=0; j < ${#START_X[@]}; j++ )) do
      NEW_START_X+=($((START_X[$j]-START_LEN)))
      NEW_START_X+=($((START_X[$j]+START_LEN)))
    done
    START_X=("${NEW_START_X[@]}")
    START_LEN=$((START_LEN/2))
  done


  for (( i=1; i<=ROWS; i++ )) do
    for (( j=1; j <=COLS; j++ )) do
      printf "%c" "${matrix[$i,$j]}"
    done
    echo ""
  done
}

# start_time=$(date "+%s.%N")
solve
# end_time=$(date "+%s.%N")
# elapsed_time=$(awk "BEGIN {printf \"%.2f\",${end_time}-${start_time}}")
# echo "Elapsed: $elapsed_time"
