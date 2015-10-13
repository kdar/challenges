T = gets.chomp.to_i
T.times do
  n = gets.chomp.to_i
  ni = gets.split.map {|n| n.to_i}

  puts(ni.inject { |product, n| product * n } % 1234567)
end
