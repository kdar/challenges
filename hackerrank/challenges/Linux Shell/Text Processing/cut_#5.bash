#!/bin/bash

ISTEST="$1"
TEST=$(echo -e "1\tNew York, New York[10]\t8,244,910\t1\tNew York-Northern New Jersey-Long Island, NY-NJ-PA MSA\t19,015,900\t1\tNew York-Newark-Bridgeport, NY-NJ-CT-PA CSA\t22,214,083\n2\tLos Angeles, California\t3,819,702\t2\tLos Angeles-Long Beach-Santa Ana, CA MSA\t12,944,801\t2\tLos Angeles-Long Beach-Riverside, CA CSA\t18,081,569\n3\tChicago, Illinois\t2,707,120\t3\tChicago-Joliet-Naperville, IL-IN-WI MSA\t9,504,753\t3\tChicago-Naperville-Michigan City, IL-IN-WI CSA\t9,729,825\n4\tHouston, Texas\t2,145,146\t4\tDallas-Fort Worth-Arlington, TX MSA\t6,526,548\t4\tWashington-Baltimore-Northern Virginia, DC-MD-VA-WV CSA\t8,718,083\n5\tPhiladelphia, Pennsylvania[11]\t1,536,471\t5\tHouston-Sugar Land-Baytown, TX MSA\t6,086,538\t5\tBoston-Worcester-Manchester, MA-RI-NH CSA\t7,601,061")

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
    echo "$line" |cut -d $'\t' -f-3
  done
}

# start_time=$(date "+%s.%N")
solve
# end_time=$(date "+%s.%N")
# elapsed_time=$(awk "BEGIN {printf \"%.2f\",${end_time}-${start_time}}")
# echo "Elapsed: $elapsed_time"
