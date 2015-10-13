<?php

$TEST = "2
3
6";

$fac = gmp_strval(gmp_fact(20));

function solve($reader) {
  fscanf($reader, "%d\n", $number);

  for ($i = 0; $i < $number; $i++) {
    fscanf($reader, "%d\n", $N);
    $fac = gmp_strval(gmp_fact($N));

    $sum = 0;
    while ($fac != 0) {
      $sum += bcmod($fac, 10);
      $fac = bcdiv($fac, 10);
    }
    echo $sum . "\n";
  }
}

function main() {
  global $argv, $argc, $TEST;
  
  // for testing purposes
  if ($argc == 2 and $argv[1] == "test") {
    $stream = fopen('php://memory', 'r+');
    fwrite($stream, $TEST);
    rewind($stream);
    solve($stream);
    return;
  }
  solve(STDIN);
}

main();
