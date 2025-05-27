module Accipere where

import           Accipere.CPU        (CPU)
import           Accipere.FileSystem
import           Accipere.GPU
import           Accipere.Misc
import           Accipere.Networking
import           Accipere.RAM        (RAM)

data Accipere = Accipere
  { cpu :: CPU,
    ram :: RAM
    -- gpu :: [GPU],
    -- net :: [Interface]
  }
