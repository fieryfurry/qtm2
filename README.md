# Quick Torrent Maker 2 (QTM2)

![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/fieryfurry/qtm2)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/fieryfurry/qtm2/rust.yml?logo=github)
![GitHub top language](https://img.shields.io/github/languages/top/fieryfurry/qtm2?color=orange&logo=rust&logoColor=orange)
[![License](https://img.shields.io/badge/license-BSD--2--Clause%20Plus%20Patent-green)](https://spdx.org/licenses/BSD-2-Clause-Patent.html)
![Platform](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey)

QTM2 is a cross-platform application that aims to integrate creating and uploading torrents to [GayTorrent](https://www.gaytor.rent/), in order to produce more quality and descriptive contents easier from most modern operating system platforms (Windows / macOS / Linux). It is written in **Rust**, a fast and memory-safe general purpose programming language, using egui and other library crates as well as fonts that may be found in the [Cargo.toml](Cargo.toml) and [LICENSE.md](LICENSE.md) files.

This is a project that is undergoing active development, so bugs are possible. Please report any in the [Issues](https://github.com/fieryfurry/qtm2/issues) section with detailed descriptions of your system configuration and steps to reproduce the bug. You may also suggest new features there to be added to the roadmap section below. 

Please observe and follow all GT [site rules](https://www.gaytor.rent/rules.php) and refer to the [FAQs](https://www.gaytor.rent/faq.php) and [help desk](https://www.gaytor.rent/helpdesk.php) for other queries/issues. Please act responsibly and respect other members of the community.

Please also note that the binary and its data structures are referred to as `qtm` in the source code only for brevity; it is not derived from nor associated with the source code or the authors/contributors of the original QTM at all; however, the UI and many functionalities are built upon its design.

[![Proton Mail](https://img.shields.io/badge/ProtonMail-8B89CC?style=for-the-badge&logo=protonmail&logoColor=white)](mailto:fiery.furry@proton.me)

## Roadmap:

### Completed:
- [x] Top/bottom UI panels
- [x] TOML configuration file (de)serialisation
- [x] Tracing
- [x] CI testing/package generation
- [x] Torrent file (de)serialisation
- [x] Image preview
- [x] UI Customisation
- [x] Password prompt
- [x] Desktop Icon

### Priority:
- [ ] Networking
- [ ] uTorrent/qBittorrent integration

### Future:
- [ ] Makefile
- [ ] Video thumbnail generator
- [ ] CLI support


## Installation
### Prebuilt binaries
The latest binaries can be found in the [Releases](https://github.com/fieryfurry/qtm2/releases) section under the Assets sub-heading. They are automatically built using [GitHub Actions](https://github.com/fieryfurry/qtm2/actions/workflows/release.yml) on each release from the source code.

### Build from source
#### Install dependencies (Linux ONLY)
Note: dependency installation _only_ tested on Ubuntu
| (Base) Distributions       | Dependency Installation                         |
|----------------------------|------------------------------------------------:|
| **Debian/Ubuntu**          | `sudo apt-get install libgtk-3-dev`             |
| **Arch Linux**             | `sudo pacman -S gtk3`                           |
| **Fedora**                 | `sudo dnf install gtk3-devel`                   |

1. Follow the instruction on the official [Rust](https://www.rust-lang.org/tools/install) website to install the Rust toolchain, including `cargo`.
2. Execute `cargo install --git https://github.com/fieryfurry/qtm2` in your terminal

## Tracing
QTM2 uses tracing extensively for debugging and error logging purposes. You may find those logs in:
| Platform     | Path                                                        |
| -------------| -----------------------------------------------------------:|
| **Windows**  | `%LOCALAPPDATA%\quick-torrent-maker-2\data\`                |
| **macOS**    | `$HOME/Library/Application Support/quick-torrent-maker-2/`  |
| **Linux**    | `$HOME/.local/share/quick-torrent-maker-2/`                 |

Please attach the relevant log in any bug report, after you have removed or censored any sensitive information within (e.g. file path). 

Note: clearing the cache will also delete all logs, as well as the configurations and previously cached .torrent files. Use caution when performing this action!

## Special Thanks
- [mitmproxy](https://mitmproxy.org/): a free and open source interactive HTTPS proxy


## License
QTM2 is licensed under the [BSD-2-Clause Plus Patent License](https://spdx.org/licenses/BSD-2-Clause-Patent.html). 

This license is designed to provide: a) a simple permissive license; b) that is compatible with the GNU General Public License (GPL), version 2; and c) which also has an express patent grant included. 
