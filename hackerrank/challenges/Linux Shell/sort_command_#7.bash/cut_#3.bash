#!/bin/bash

ISTEST="$1"
TEST=$(echo -e "Hello\nWorld\nhow are you\n")

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
  while doread line; do
    echo -n "$line" |cut -b2-7
  done
}

# start_time=$(date "+%s.%N")
solve
# end_time=$(date "+%s.%N")
# elapsed_time=$(awk "BEGIN {printf \"%.2f\",${end_time}-${start_time}}")
# echo "Elapsed: $elapsed_time"
