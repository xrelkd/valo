<h1 align="center">Valo</h1>

<p align="center">
    A Program to Control Backlights (and other Hardware Lights) in GNU/Linux.
</p>

<p align="center">
    <a href="https://github.com/xrelkd/valo/releases"><img src="https://img.shields.io/github/v/release/xrelkd/valo.svg"></a>
    <a href="https://github.com/xrelkd/valo/actions?query=workflow%3ARust"><img src="https://github.com/xrelkd/valo/workflows/Rust/badge.svg"></a>
    <a href="https://github.com/xrelkd/valo/actions?query=workflow%3ARelease"><img src="https://github.com/xrelkd/valo/workflows/Release/badge.svg"></a>
    <a href="https://github.com/xrelkd/valo/blob/master/LICENSE"><img alt="GitHub License" src="https://img.shields.io/github/license/xrelkd/valo"></a>
</p>

**[Installation](#installation) | [Usage](#usage) | [License](#license)**

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
