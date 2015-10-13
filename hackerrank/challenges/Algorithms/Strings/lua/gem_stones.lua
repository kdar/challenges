TEST = [[3
abcdde
baccd
eeabg]]

function stringio(s)
  f = assert(io.tmpfile())
  f:write(s)
  f:flush()
  f:seek("set", 0)
  return f
end

function setDefault(t, d)
  local mt = {__index = function () return d end}
  setmetatable(t, mt)
end

function solve(input)
  test_cases = tonumber(input:read("*l"))

  counts = {}
  setDefault(counts, 0)
  for i=1, test_cases do
    line = input:read("*l")

    mark = {}
    for x=1, #line do
      local c = line:sub(x,x)
      mark[c] = 1
    end

    for key,value in pairs(mark) do
      counts[key] = counts[key] + 1
    end
  end

  count = 0
  for key,value in pairs(counts) do
    if value == test_cases then
      count = count + 1
    end
  end

  print(count)
end

function main(...)
  if arg[1] == "test" then
    f = stringio(TEST)
    solve(f)
    f:close()
    return
  end

  solve(io.stdin)
end

main(...)
