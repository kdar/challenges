# https://www.hackerrank.com/challenges/the-love-letter-mystery

def get_count(s)
  count = 0
  for i in 0...(s.length/2) do
    if s[i] != s[s.length-i-1] then
      count += (s[i].ord-s[s.length-i-1].ord).abs
    end
  end
  count
end

T = gets.chomp.to_i
T.times do
  count = 0
  puts(get_count(gets.chomp))
end
