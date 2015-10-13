#!/bin/bash

ISTEST="$1"
TEST=$(echo -e "")

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
  # doread test_cases;
  # for (( i=1; i<=test_cases; i++ )); do
  #   doread line
  # done
}

# start_time=$(date "+%s.%N")
solve
# end_time=$(date "+%s.%N")
# elapsed_time=$(awk "BEGIN {printf \"%.2f\",${end_time}-${start_time}}")
# echo "Elapsed: $elapsed_time"
