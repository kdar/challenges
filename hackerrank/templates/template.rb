require 'stringio'

TEST = ""

def solve(input)
  #string = input.gets.chomp.downcase
end

def main()
  # for testing purposes
  if ARGV.length == 1 && ARGV[0] == "test"
    solve(StringIO.new(TEST))
    return
  end

  solve($stdin)
end

if __FILE__ == $0
  main()
end
