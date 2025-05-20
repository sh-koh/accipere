module Accipere (accipere) where

import Accipere.CPU
import Accipere.FileSystem
import Accipere.GPU
import Accipere.Misc
import Accipere.Networking
import Accipere.RAM

accipere :: [String] -> IO ()
accipere [] = putStrLn "no arguments provided."
accipere args'@(arg : args) = do
  case arg of
    "--all" -> putStrLn arg
    "--none" -> putStrLn undefined
    _ -> putStrLn "pog"
