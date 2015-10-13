#!/bin/bash

ISTEST="$1"
TEST=$(echo -e "6\naaabbb\nab\nabc\nmnop\nxyyx\nxaxbbbxx")

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
  doread test_cases;

  for (( i=1; i<=test_cases; i++ )); do
    doread line

    if [ `expr ${#line} % 2` -eq 1 ]; then
      echo "-1"
    else
      size=`expr ${#line} / 2`;
      string1="${line:0:$size}";
      string2="${line:$size:$size+$size}";

      unset counts1
      unset counts2
      declare -A counts1
      declare -A counts2

      for (( x=0; x < ${#string1}; x++ )); do
        index="${string1:x:1}"
        counts1[$index]=$((${counts1[$index]:-0} + 1));
      done

      for (( x=0; x < ${#string2}; x++ )); do
        index="${string2:x:1}"
        counts2[$index]=$((${counts2[$index]:-0} + 1));
      done

      deletions=0
      for c in "${!counts2[@]}"; do
        current=$((${counts2[${c}]:-0} - ${counts1[${c}]:-0}));
        if [ "$current" -gt "0" ]; then
          deletions=$((${deletions} + $current));
        fi
      done
      echo "$deletions"
    fi
  done
}

# start_time=$(date "+%s.%N")
solve
# end_time=$(date "+%s.%N")
# elapsed_time=$(awk "BEGIN {printf \"%.2f\",${end_time}-${start_time}}")
# echo "Elapsed: $elapsed_time"
