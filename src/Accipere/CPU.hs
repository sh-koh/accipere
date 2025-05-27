module Accipere.CPU where

data CPU = CPU
  { vendor    :: String,
    modelName :: String,
    frequency :: Double,
    cores     :: Int,
    threads   :: Int
  }

instance Show CPU where
  show (CPU {..}) =
    "[CPU]:"
      <> "\n\tVendor: "
      <> vendor
      <> "\n\tModel: "
      <> modelName
      <> "\n\tFrequency: "
      <> show frequency
      <> "\n\tCores: "
      <> show cores
      <> "\n\tThreads: "
      <> show threads

getCPUInfo :: IO CPU
getCPUInfo = do
  cpuinfo <- readFile "/proc/cpuinfo"
  let cleanedUp = map go $ lines cpuinfo
  return $
    CPU
      { vendor = cleanedUp !! 1,
        modelName = cleanedUp !! 4,
        frequency = read (cleanedUp !! 7) :: Double,
        cores = read (cleanedUp !! 12) :: Int,
        threads = read (cleanedUp !! 10) :: Int
      }
  where
    go :: String -> String
    go [] = []
    go line =
      case dropWhile (/= ':') line of
        []         -> ""
        (_ : rest) -> dropWhile (== ' ') rest
