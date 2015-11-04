def sum_terms(n)
  (1..n).reduce(0) {|cum,x| cum + (x*x + 1)}
end
