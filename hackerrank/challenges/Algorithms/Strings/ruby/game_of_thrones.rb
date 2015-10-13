require 'stringio'

TEST = "racecar"

def solve(input)
  string = input.gets.chomp.downcase

  count = Hash.new {0}
  string.each_char do |i|
    count[i] += 1
  end

  odd_count = 0
  count.each do |key,array|
    if count[key] % 2 != 0
      odd_count+=1
    end

    if odd_count > 1
      puts "NO"
      return
    end
  end

  puts "YES"
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
