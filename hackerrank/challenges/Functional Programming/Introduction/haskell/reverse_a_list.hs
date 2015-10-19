rev [] = []
rev l = last l : rev (init l)

-- way #2
-- rev [] = []
-- rev (x:xs) = (rev xs) ++ [x]
