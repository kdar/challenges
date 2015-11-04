
def rot13(secret_messages)
  secret_messages.map {|word| word.tr('A-Za-z', 'N-ZA-Mn-za-m') }
end
