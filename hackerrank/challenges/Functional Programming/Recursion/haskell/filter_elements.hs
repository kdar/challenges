import qualified Data.IntMap as IntMap
import Control.Monad

filterElements k [] cnts seen = filter (\x -> (IntMap.findWithDefault 0 x cnts) >= k) seen
filterElements k (x:xs) cnts seen =
  case IntMap.lookup x cnts of
      Nothing -> filterElements k xs (IntMap.insert x 1 cnts) (x:seen)
      Just n -> filterElements k xs (IntMap.insert x (n+1) cnts) seen

main = do
  t <- readLn
  replicateM_ t $ do
     input1 <- getLine
     input2 <- getLine
     let nk = map (read::String->Int) $ words input1
         xs = map (read::String->Int) $ words input2
         filtered = filterElements (last nk) xs IntMap.empty []
         answer = if not $ null filtered then unwords $ map show $ reverse filtered else "-1"

     putStrLn $ answer


-- method 2
-- import Control.Monad
-- import Control.Applicative
-- import Data.List
-- import Data.Function
--
-- main = do
--   t <- read <$> getLine
--   replicateM_ t $ do
--          [_, k] <- map read .words <$> getLine
--          l <- words <$> getLine
--          case map fst .sortBy (compare `on` snd) .map head .filter ((>=k) .length) .groupBy (on (==) fst) .sort $ zip l [0..] of
--            [] -> putStrLn "-1"
--            ans -> putStrLn $ unwords ans
