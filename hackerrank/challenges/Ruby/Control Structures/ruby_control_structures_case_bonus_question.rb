
def identify_class(obj)
  case obj
  when Hacker
    puts "It's a #{obj.class.name}!"
  when Submission
    puts "It's a #{obj.class.name}!"
  when TestCase
    puts "It's a #{obj.class.name}!"
  when Contest
    puts "It's a #{obj.class.name}!"
  else
    puts "It's an unknown model"
  end
end
