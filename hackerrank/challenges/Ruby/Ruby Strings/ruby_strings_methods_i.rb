def process_text(arr)
  arr.map {|x| x.strip}.join " "
end

puts process_text(["Hi, \n", " Are you having fun?    "])
