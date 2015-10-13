<?php

$TEST = "";

function solve($reader) {
  // fscanf($reader, "%d\n", $test_cases);
  // for ($i = 0; $i < $test_cases; $i++) {
  //   fscanf($reader, "%s\n", $line);
  // }
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
