def serial_average(s)
  average = sprintf('%.02f', (s[4,5].to_f + s[10,5].to_f) / 2.0)
  "#{s[0,3]}-#{average}"
end

puts serial_average('002-10.00-20.00')
