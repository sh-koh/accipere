module Accipere.RAM where

-- /proc/meminfo
-- 1 - MemTotal:       32789044 kB
-- 2 - MemFree:        16709236 kB
-- 3 - MemAvailable:   27133040 kB
-- 15 - SwapTotal:      16394236 kB
-- 16 - SwapFree:       16394236 kB

data RAM = RAM
  { total     :: Int,
    free      :: Int,
    available :: Int,
    swapTotal :: Int,
    swapFree  :: Int
  }

instance Show RAM where
  show (RAM {..}) =
    "[RAM]:"
      <> "\n\tTotal: "
      <> show total
      <> "\n\tFree: "
      <> show free
      <> "\n\tAvailable: "
      <> show available
      <> "\n\tSwap: "
      <> "\n\t\tTotal: "
      <> show swapTotal
      <> "\n\t\tFree: "
      <> show swapFree

getRAMInfo :: IO RAM
getRAMInfo = do
  meminfo <- readFile "/proc/meminfo"
  let cleanedUp = map go $ lines meminfo
  return $
    RAM
      { total = read (cleanedUp !! 0) :: Int,
        free = read (cleanedUp !! 1) :: Int,
        available = read (cleanedUp !! 2) :: Int,
        swapTotal = read (cleanedUp !! 14) :: Int,
        swapFree = read (cleanedUp !! 15) :: Int
      }
  where
    go :: String -> String
    go [] = []
    go line =
      case words line of
        (_ : value : _) -> value
        []              -> ""
        _               -> ""
