import Text.Printf

task1 :: Int -> Int
task1 n = let ak = maximum [i | i <- [0..n], isPrime i]
              al = head [i | i <- [n..], isPrime i]
          in min (n - ak) (al - n)

isPrime :: Int -> Bool
isPrime a
    | a <= 1 = False
    | otherwise = all (\i -> a `mod` i /= 0) [2..floor (sqrt (fromIntegral a))]

main :: IO ()
main = do
    k <- readLn
    printf "%d\n" (task1 k)