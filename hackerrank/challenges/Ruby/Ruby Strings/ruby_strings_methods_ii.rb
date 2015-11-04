def mask_article(str, words)
  words.each_entry do |x|
    str = str.gsub(Regexp.new("(#{x})"), strike('\1'))
  end
  str
end

def strike(s)
  "<strike>#{s}</strike>"
end

#puts mask_article("Hello World! This is crap!", ["crap"])
