def count_multibyte_char(s)
  c = 0
  s.each_char {|x| if x.bytesize > 1 then c += 1 end}
  c
end

#puts count_multibyte_char('¥1000')
