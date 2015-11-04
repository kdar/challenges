def group_by_marks(marks, n)
  marks.group_by {|k,v| if v < n then "Failed" else "Passed" end}
end
