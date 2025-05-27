# 💻 Accipere (WIP)
Accipere (english for "get" in latin) is an quick system information prompter
for GNU/Linux written in Haskell.

## 📔 Usage
```bash
$ accipere
          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖             user@hostname
          ▜███▙       ▜███▙  ▟███▛
           ▜███▙       ▜███▙▟███▛              OS: NixOS 25.11 (Xantusia) x86_64
            ▜███▙       ▜██████▛               Host: MS-7B86 (1.0)
     ▟█████████████████▙ ▜████▛     ▟▙         Kernel: Linux 6.14.7-zen1
    ▟███████████████████▙ ▜███▙    ▟██▙        Uptime: 4 hours, 59 mins
           ▄▄▄▄▖           ▜███▙  ▟███▛        Packages: 1450 (nix-system), 3298 (nix-user)
          ▟███▛             ▜██▛ ▟███▛         Shell: bash 5.2.37
         ▟███▛               ▜▛ ▟███▛          WM: Hyprland 0.49.0
▟███████████▛                  ▟██████████▙    Theme: Base16Kvantum [Qt], adw-gtk3 [GTK2/3/4]
▜██████████▛                  ▟███████████▛    Icons: Papirus-Dark [GTK2/3/4]
      ▟███▛ ▟▙               ▟███▛             Font: FiraMono Nerd Font (10pt) [GTK2/3/4]
     ▟███▛ ▟██▙             ▟███▛              Cursor: BreezeX-Black (32px)
    ▟███▛  ▜███▙           ▝▀▀▀▀               Terminal: kitty 0.42.0
    ▜██▛    ▜███▙ ▜██████████████████▛         Terminal Font: FiraCodeNFM-Reg (10pt)
     ▜▛     ▟████▙ ▜████████████████▛          CPU: AMD Ryzen 7 2700X (16) @ 3.95 GHz
           ▟██████▙       ▜███▙                GPU: NVIDIA GeForce RTX 2070
          ▟███▛▜███▙       ▜███▙               Memory: 4.08 GiB / 31.27 GiB (13%)
         ▟███▛  ▜███▙       ▜███▙              Swap: 0 B / 15.63 GiB (0%)
         ▝▀▀▀    ▀▀▀▀▘       ▀▀▀▘              Disk (/): 73.76 GiB / 168.38 GiB (44%) - ext4
                                               Disk (/home): 47.60 GiB / 59.50 GiB (80%) - ext4
                                               Local IP (enp34s0): 192.168.1.10/24
                                               Locale: en_US.UTF-8
```

## ⚙️ Configuration
> Not yet implemented

A config file is supported if you want to tweak some settings.<br>
There is an exemple file in this repo in the `example` directory.<br>

**Config file location:** `$XDG_CONFIG_HOME/accipere/config.toml`

#### Settings
```toml
section_separators = '-'

[logo]
hidden = false
center = true
# Either plain text or a path to a txt file.
custom = "
    __
   /  \
   \__/
"

# The box containing all informations, excluding the logo
[box]
sides = ["-", "|", "-", "|"] # [ up, right, down, left ]
corners = ["+", "+", "+", "+"] # [ top-left, top-right, down-right, down-left ]

[cpu.model]
hidden = false
color = `#642424`

...

```

## 🔨 Building

#### `nix`:
```bash
$ nix build .#
```

#### `cabal`:
```bash
$ cabal build
```

#### `stack`:
```bash
$ stack build
```
