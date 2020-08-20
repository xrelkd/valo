# Valo: A Program to Control Backlights (and other Hardware Lights) in GNU/Linux

![Build](https://github.com/xrelkd/valo/workflows/Build/badge.svg)

- [Introduction](#introduction)
- [Usage](#usage)
- [License](#license)

## Introduction

Valo is a program to control backlights and other lights under GNU/Linux.

## Usage

**Supported devices** :

| Device     |                    |
| ---------- | ------------------ |
| `keyboard` | keyboard backlight |
| `screen`   | screen backlight   |

**Note** : Replace `<device>` with devices mentioned above.

| Command                           | Example                           |                                                            |
| --------------------------------- | --------------------------------- | ---------------------------------------------------------: |
| `valo <device> get`               | `valo keyboard get`               |            Get the current `keyboard` backlight brightness |
| `valo <device> get-percentage`    | `valo keyboard get-percentage`    | Get the current `keyboard` backlight brightness in percent |
| `valo <device> set-percentage 42` | `valo keyboard set-percentage 42` |       Set `keyboard` backlight brightness as 42 percentage |
| `valo <device> up 5`              | `valo keyboard up 5`              |      Increase `keyboard` backlight brightness by 5 percent |
| `valo <device> down 5`            | `valo keyboard down 5`            |      Decrease `keyboard` backlight brightness by 5 percent |
| `valo <device> max`               | `valo keyboard max`               |                 Set `keyboard` backlight brightness as max |
| `valo <device> off`               | `valo keyboard off`               |                              Turn off `keyboard` backlight |

## License

Valo is licensed under the GNU General Public License version 3. See [LICENSE](./LICENSE) for more information.
