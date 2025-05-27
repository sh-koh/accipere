module Main where

import Accipere
import Accipere.CPU (getCPUInfo)
import Accipere.RAM (getRAMInfo)
import System.Environment (getArgs)
import System.Exit (exitFailure)

accipere :: [String] -> IO ()
accipere args = do
  case args of
    [] -> printAll
    ["--help"] -> putStrLn "Need help?"
    ["--cpu"] -> getCPUInfo >>= print
    ["--ram"] -> getRAMInfo >>= print
    arg : _ -> putStrLn $ "[!] Argument is not valid: " <> arg
  where
    printAll :: IO ()
    printAll = do
      cpu <- getCPUInfo
      ram <- getRAMInfo
      print cpu >> print ram

main :: IO ()
main = do
  args <- getArgs
  accipere args
  return ()
