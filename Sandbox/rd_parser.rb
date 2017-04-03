
def rd_parse(text)
  rd_tree(text.chars.each)
end

def rd_tree(chars)
  list = []
  word = ""
  loop do
    case c = chars.next
    when /\s/
      list << word unless word.empty?
      word = ""
    when "("
      list << word unless word.empty?
      word = ""
      list << rd_tree(chars)
    when ")"
      list << word unless word.empty?
      return list
    else
      word << c
    end
  end
  list << word unless word.empty?
  list
rescue StopIteration
  raise "missing paren?"
end

p "(add 5 (add 4 1) 3)"
p rd_parse "(add 5 (add 4 1) 3)"
p rd_parse "add 5 (add 4 1) 3)"
p rd_parse "add 5 ((add 4 1)) 3"
p rd_parse "((add 5 (add 4 1) 3))"

p "add ((add 5 5)) 3"
p rd_parse "add ((add 5 5)) 3"
