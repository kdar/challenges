HelloWorld = {}

function HelloWorld.hello(input)
  if input == nil then
    input = 'world'
  end
  return 'Hello, ' .. input .. '!'
end

return HelloWorld
