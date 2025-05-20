module Main where

import Accipere (accipere)
import System.Environment (getArgs)

main :: IO ()
main = do
  args <- getArgs
  accipere args
  return ()
