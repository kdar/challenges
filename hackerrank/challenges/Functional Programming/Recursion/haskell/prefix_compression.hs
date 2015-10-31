import Text.Printf

common [] _ = []
common _ [] = []
common (x1:xs1) (x2:xs2) = if x1 == x2 then x1:common xs1 xs2 else []

main = do
  input1 <- getLine
  input2 <- getLine
  let c = common input1 input2
      i1 = drop (length c) input1
      i2 = drop (length c) input2
  printf "%d %s\n" (length c) c
  printf "%d %s\n" (length i1) i1
  printf "%d %s\n" (length i2) i2
