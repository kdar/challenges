function isUnique(n, f) {
  var seen = {};

  for (var i = 0; i < n; i++) {
    if (f[i] in seen) {
      return false;
    }
    seen[f[i]] = true;
  }
  return true;
}

function processData(input) {
  var lines = input.split('\n');
  var n = parseInt(lines.shift());

  var l = lines.shift().split(' ');
  var f = [];
  for (var x = 0; x < l.length; x++) {
    f[x] = parseInt(l[x]);
  }

  if (isUnique(n, f)) {
    console.log("YES");
  } else {
    console.log("NO");
  }
}

process.stdin.resume();
process.stdin.setEncoding("ascii");
_input = "";
process.stdin.on("data", function (input) {
  _input += input;
});
process.stdin.on("end", function () {
  processData(_input);
});
