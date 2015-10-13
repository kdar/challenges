#!/bin/bash

ISTEST="$1"
TEST="New York is a state in the Northeastern and Mid-Atlantic regions of the United States.
New York is the 27th-most extensive, the third-most populous populated of the 50 United States.
New York is bordered by New Jersey and Pennsylvania to the south.
About one third of all the battles of the Revolutionary War took place in New York.
Henry Hudson's 1609 voyage marked the beginning of European involvement with the area."

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
    result=$line;
    # http://www.fvue.nl/wiki/Bash:_Passing_variables_by_reference
    if unset -v "$__resultvar"; then # Unset & validate varname
      eval $__resultvar=\"\$result\"
    fi
  else
    echo "$line";
  fi

  return $status
}

solve() {
  while doread line; do
    echo "$line" |cut -b13-
  done
}

# start_time=$(date "+%s.%N")
solve
# end_time=$(date "+%s.%N")
# elapsed_time=$(awk "BEGIN {printf \"%.2f\",${end_time}-${start_time}}")
# echo "Elapsed: $elapsed_time"
