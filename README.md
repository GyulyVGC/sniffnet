<p align="center"><a href="https://www.sniffnet.net"><img alt="" src="https://github.com/GyulyVGC/sniffnet/blob/main/resources/repository/header_repository.png?raw=true" width="100%"/></a></p>

<p align="center"> 
<a href="https://github.com/GyulyVGC/sniffnet/blob/main/LICENSE-APACHE"><img alt="" src="https://img.shields.io/crates/l/sniffnet?&color=orange"/></a>
&nbsp;
<a href="https://crates.io/crates/sniffnet"><img alt="" src="https://img.shields.io/crates/v/sniffnet?&logo=rust&color=blue"/></a> <br>
</p>

<p align="center"> 
Application to comfortably monitor your Internet traffic <br>
Multithreaded, cross-platform, reliable <br>
🌐 <a href="https://www.sniffnet.net">www.sniffnet.net</a>
</p>

<div align="center">

Graphical interface translated in:<br>
🇬🇧&nbsp;&nbsp;🇩🇪&nbsp;&nbsp;🇬🇷&nbsp;&nbsp;🇪🇦&nbsp;&nbsp;🇮🇷&nbsp;&nbsp;🇫🇷&nbsp;&nbsp;🇮🇹&nbsp;&nbsp;🇰🇷&nbsp;&nbsp;🇵🇱&nbsp;&nbsp;🇵🇹&nbsp;&nbsp;🇷🇴&nbsp;&nbsp;🇷🇺&nbsp;&nbsp;🇸🇪&nbsp;&nbsp;🇹🇷&nbsp;&nbsp;🇺🇦&nbsp;&nbsp;🇨🇳<br>

</div>

<p>
<a href="#x">
<img alt="" src="https://github.com/GyulyVGC/sniffnet/blob/main/resources/repository/hr.png?raw=true" width="100%"/>
</a>
</p>

<div align="center"><img alt="" src="https://github.com/GyulyVGC/sniffnet/blob/main/resources/repository/pages/overview_page.png?raw=true" width="100%"/></div>

<div align="center">
<img alt="" src="https://github.com/GyulyVGC/sniffnet/blob/main/resources/repository/pages/inspect_page.png?raw=true" width="49%"/>
<img alt="" src="https://github.com/GyulyVGC/sniffnet/blob/main/resources/repository/pages/notifications_page.png?raw=true" width="49%"/>
</div>

<p>
<a href="#x">
<img alt="" src="https://github.com/GyulyVGC/sniffnet/blob/main/resources/repository/hr.png?raw=true" width="100%"/>
</a>
</p>


## _Help fund Sniffnet's development_ 💖

Sniffnet is **completely free, open-source software** which needs lots of effort and time to develop and maintain.

If you appreciate Sniffnet, [**consider sponsoring**](https://github.com/sponsors/GyulyVGC):
***your support will allow me to dedicate more and more time to this project***,
constantly expanding it including **new features and functionalities**.<br/>

A special mention goes to these awesome organizations and folks who are sponsoring Sniffnet:

<p align="center">
<a href="https://github.com/github"><img src="https://avatars.githubusercontent.com/github?v=4" width="75px" alt="github"/></a>&nbsp;&nbsp;
<a href="https://github.com/0x0177b11f"><img src="https://avatars.githubusercontent.com/0x0177b11f?v=4" width="75px" alt="tiansheng li"/></a>
</p>


## Installation

You can install Sniffnet in one of the following ways:


<details>

  <summary>from GitHub releases&ensp;<img alt="" src="https://img.shields.io/github/downloads/gyulyvgc/sniffnet/total?color=success&logo=github"/></summary>

  You can install Sniffnet through the installers available in the [latest release](https://github.com/GyulyVGC/sniffnet/releases/latest). <br>
  Choose from a Windows installer, a macOS disk image, a DEB package, or an RPM package (depending on your operating system). <br>
  Below, for your convenience, you can find the direct links to the downloads.

  ### Windows

  - [Windows 64-bit](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_Windows_64-bit.msi)
  - [Windows 32-bit](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_Windows_32-bit.msi) (only for older architectures)

  ### macOS
  
  - [macOS Intel](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_macOS_Intel.dmg)
  - [macOS Apple Silicon](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_macOS_AppleSilicon.dmg)

  ### Linux DEB-based

  - [Linux amd64](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_LinuxDEB_amd64.deb)
  - [Linux arm64](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_LinuxDEB_arm64.deb)
  - [Linux i386](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_LinuxDEB_i386.deb) (only for older architectures)
  - [Linux armhf](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_LinuxDEB_armhf.deb) (only for older architectures)

### Linux RPM-based

  - [Linux x86_64](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_LinuxRPM_x86_64.rpm)
  - [Linux aarch64](https://github.com/GyulyVGC/sniffnet/releases/latest/download/Sniffnet_LinuxRPM_aarch64.rpm)

</details>


<details>

  <summary>from Crates.io&ensp;<img alt="" src="https://img.shields.io/crates/d/sniffnet?color=success&label=downloads&logo=rust"/></summary>

Follow this method only if you have [Rust installed](https://www.rust-lang.org/tools/install) on your machine. <br>
In this case, the application binary can be built and installed with:

```sh
cargo install sniffnet
```

</details>


<details>

  <summary>from Homebrew</summary>

  You can install [Sniffnet Homebrew package](https://github.com/Homebrew/homebrew-core/pkgs/container/core%2Fsniffnet) with:

  ```sh
brew install sniffnet
```

</details>


<details>

  <summary>on Arch Linux</summary>

  You can install Sniffnet community package via [pacman](https://wiki.archlinux.org/title/Pacman):

  ```sh
pacman -S sniffnet
```

</details>

## Required dependencies

Depending on your operating system, you may need to install some dependencies to run Sniffnet:

<details>

  <summary>Windows dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474292-d84f2a96-f445-40ac-8930-9d0f00a3c2bb.png" width="35px"/></summary>

  In order to correctly build and run Sniffnet on Windows systems you need to:

  - Install [Npcap](https://npcap.com/#download), making sure to check the box `Install Npcap in WinPcap API-compatible Mode` during the installation.

  - Download the [Npcap SDK](https://npcap.com/#download).

  - Add the SDK's ```/Lib/x64``` (or ```/Lib```) folder to your ```LIB``` environment variable.

</details>


<details>

  <summary>Linux dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474239-c48d37af-d4c1-4a94-9207-0d46c6d75f1f.png" width="35px"/></summary>

  - On [DEB-based](https://en.wikipedia.org/wiki/List_of_Linux_distributions#Debian-based) distributions:
    - `libpcap-dev`
    - `libasound2-dev`
    - `libfontconfig1-dev`
  - On [RPM-based](https://en.wikipedia.org/wiki/List_of_Linux_distributions#RPM-based) distributions:
    - `libpcap-devel`
    - `alsa-lib-devel`
    - `fontconfig-devel`

  Note that if you are not running as root, you need to set capabilities to inspect a network adapter:

```sh
sudo setcap cap_net_raw,cap_net_admin=eip <your/Sniffnet/executable/path>
```

  Alternatively you can run the app with sudo privileges:

```sh
sudo sniffnet
```

</details>


<details>

  <summary>MacOS dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474398-7637e269-3e92-44bc-87c0-8ea18ca95693.png" width="35px"/></summary>

  MacOS natively has all the dependencies you need to build and run Sniffnet!

</details>


## Features

- 💻 choose a network adapter of your PC to inspect
- 🏷️ select a set of filters to apply to the observed traffic
- 📖 view overall statistics about your Internet traffic
- 📈 view real-time charts about traffic intensity (bytes and packets per second, incoming and outgoing)
- 🌐 get details about domain names and network providers of the hosts you are exchanging traffic with
- 🏠 identify connections in your local network
- 🌍 get information about the country of the remote hosts (IP geolocation)
- ⭐ save your favorite network hosts
- 🔉 set custom notifications to inform you when defined network events occur
- 🎨 choose the style that fits you the most from 4 different available themes
- 🕵️ inspect each of your network connections in real time
- 📁 save complete textual reports with detailed information for each network connection:
  * source and destination IP addresses
  * source and destination ports
  * carried protocols
  * amount of exchanged packets and bytes
  * initial and final timestamp of information exchange
- ... and more!


## IP geolocation and network providers (ASN)

<details>

  <summary>See details</summary>

  <br>

  Geolocation and network providers (ASN) refer to the remote IP address of each connection. They are retrieved performing lookups against [MMDB files](https://maxmind.github.io/MaxMind-DB/):

  > **Note**
  > 
  > The MMDB (MaxMind database) format has been developed especially for IP lookup.<br>
  > It is optimized to perform lookups on data indexed by IP network ranges quickly and efficiently.<br>
  > It permits the best performance on IP lookups, and it's suitable for use in a production environment.
  > 
  > This product includes GeoLite2 data created by MaxMind, available from <a href="https://www.maxmind.com">https://www.maxmind.com </a>
  
  This file format potentially allows Sniffnet to execute hundreds of different IP lookups in a matter of a few milliseconds.

</details>


## Supported application layer protocols

<details>

  <summary>See details</summary>
  
  <br>
  
  Application layer protocols are inferred from the transport port numbers,
  following the convention maintained by [IANA](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml).

  Please, remember that this is just a convention:

  > **Warning**
  > 
  > The Internet Assigned Numbers Authority (IANA) is responsible for maintaining
  > the official assignments of port numbers for specific uses. <br>
  > However, many unofficial uses of well-known port numbers occur in practice.

  The following table reports the port-to-service mappings used by Sniffnet,
  chosen from the most common assignments by IANA.
  
<div align="center">

|Port number(s)|Application protocol  |  Description |
|--|--|--|
| 20, 21 | FTP |File Transfer Protocol |
|22|SSH |Secure Shell |
|23|Telnet |Telnet |
|25|SMTP |Simple Mail Transfer Protocol |
|49|TACACS |Terminal Access Controller Access-Control System |
|53|DNS |Domain Name System |
|67, 68|DHCP |Dynamic Host Configuration Protocol |
|69|TFTP |Trivial File Transfer Protocol |
|80, 8080|HTTP |Hypertext Transfer Protocol |
|109, 110|POP |Post Office Protocol |
|123|NTP |Network Time Protocol |
|137, 138, 139|NetBIOS |NetBIOS |
|143, 220|IMAP |Internet Message Access Protocol |
|161, 162, 199|SNMP |Simple Network Management Protocol |
|179|BGP |Border Gateway Protocol |
|389|LDAP |Lightweight Directory Access Protocol |
|443|HTTPS |Hypertext Transfer Protocol over SSL/TLS |
|636|LDAPS |Lightweight Directory Access Protocol over TLS/SSL |
|989, 990|FTPS |File Transfer Protocol over TLS/SSL |
|993|IMAPS |Internet Message Access Protocol over TLS/SSL |
|995|POP3S |Post Office Protocol 3 over TLS/SSL |
|1900|SSDP |Simple Service Discovery Protocol |
|5222|XMPP |Extensible Messaging and Presence Protocol |
|5353|mDNS |Multicast DNS |

</div>

</details>


## Keyboard shortcuts

<details>

  <summary>See details</summary>

<br>

Some keyboard shortcuts are available to improve the efficiency of use and the overall user experience.

If you want to suggest a different key combination for one of the existing shortcuts or if you want to propose a new shortcut,
have a look at [this](https://github.com/GyulyVGC/sniffnet/issues/97) issue.

The currently usable hotkeys are reported in the following.

> **Note**
>
> On macOS, use the `cmd` key instead of `ctrl`

<div align="center">

| Event | Shortcut keys |
|--|--|
| Quit the application | `ctrl+Q` |
| Open full report | `ctrl+O` |
| Open settings | `ctrl+,` |
| Clear all notifications | `ctrl+D` |
| Interrupt the ongoing analysis | `ctrl+backspace` |
| Start the analysis and confirm modal actions | `enter` |
| Close settings and modal popups | `esc` |
| Switch from a tab to the next (or previous) one | `tab` (or `shift+tab`) |
| Change inspect connections page to the next (or previous) one | `ctrl+rightArrow` (or `ctrl+leftArrow`) |

</div>

</details>


## Troubleshooting

<details>

  <summary>See details</summary>

### Missing dependencies

Most of the errors that can occur are likely due to your system missing required `pcap` dependencies,
necessary to correctly analyze a network adapter. <br>
Check the [required dependencies](#required-dependencies) section for instructions on how to proceed.

Note that most Linux system also need this dependency (required to build the library used to play sounds):

```sh
sudo apt-get install libasound2-dev
```

Some Linux systems also need `libfontconfig`, see issue [#18](https://github.com/GyulyVGC/sniffnet/issues/18) for a reference.

> **Note**
>
> View issues labeled with [`missing-dependencies`](https://github.com/GyulyVGC/sniffnet/issues?q=is%3Aissue+label%3A%22missing+dependency%22+) to see how those problems have been solved by others.

### Installers incompatibilities

If you have problems after having installed Sniffnet through the provided installers,
it could be due to your OS not being compatible with the pre-built binaries I generated for you. <br>
Reach out to me, and I'll try to generate an installer for your specific operating system.

> **Warning**
>
> The DEB package for Linux is built on the latest version of Ubuntu and in some cases may not be compatible with Debian. <br/>
> See issue [#199](https://github.com/GyulyVGC/sniffnet/issues/199) for a reference.

### Rendering problems

In some cases, especially if you are running on an old architecture, the `wgpu` default renderer used by [iced](https://github.com/iced-rs/iced)
may cause some problems that could prevent you from running Sniffnet. <br>
In this case, you can try building the application from the [`glow-renderer`](https://github.com/GyulyVGC/sniffnet/tree/glow-renderer) 
branch, which uses the `glow` renderer.

> **Note**
>
> View issues labeled with [`renderer`](https://github.com/GyulyVGC/sniffnet/issues?q=is%3Aissue+label%3Arenderer) to see how those problems have been solved by others.

### ***In any case don't hesitate to [open an issue](https://github.com/GyulyVGC/sniffnet/issues), and I will do my best to help you!***

</details>


## Acknowledgements

- A big shout-out to [all the contributors](https://github.com/GyulyVGC/sniffnet/blob/main/CONTRIBUTORS.md) of Sniffnet!

- The graphical user interface has been realized with [iced](https://github.com/iced-rs/iced), a cross-platform GUI library for Rust focused on simplicity and type-safety

<p align="center"><a href="https://github.com/iced-rs/iced"><img alt="" src="https://user-images.githubusercontent.com/100347457/219339409-0a44722b-416d-410b-93a4-8b0e84c0031d.svg" width="50%"/></a></p>

- Last but not least, thanks to [every single stargazer](https://github.com/GyulyVGC/sniffnet/stargazers): all forms of support made it possible to keep improving Sniffnet!

<p align="center"><a href="https://github.com/GyulyVGC/sniffnet/stargazers"><img alt="" src="https://reporoster.com/stars/dark/GyulyVGC/sniffnet"/></a></p>
