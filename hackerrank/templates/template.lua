TEST = [[]]

function stringio(s)
  f = assert(io.tmpfile())
  f:write(s)
  f:flush()
  f:seek("set", 0)
  return f
end

function solve(input)
  -- test_cases = tonumber(input:read("*l"))
  -- for i=1, test_cases do
  --   line = input:read("*l")
  -- end
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
