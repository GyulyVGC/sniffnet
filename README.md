<p align="center"><a href="https://github.com/GyulyVGC/sniffnet"><img alt="Sniffnet" src="https://user-images.githubusercontent.com/100347457/211192693-21a5dc79-c7bd-4eb7-8c80-4d954d28b9e2.png" width="100%"/></a></p>

<p align="center"> 
<a href="https://github.com/GyulyVGC/sniffnet/blob/main/LICENSE-APACHE"><img alt="" src="https://img.shields.io/crates/l/sniffnet?&color=orange"/></a>
&nbsp;
<a href="https://crates.io/crates/sniffnet"><img alt="" src="https://img.shields.io/crates/v/sniffnet?&logo=rust&color=blue"/></a> <br>
</p>

<p align="center"> 
Application to comfortably monitor your network traffic <br>
Multithreaded, cross-platform, reliable
</p>

<hr>

<p align="center"><img alt="" src="https://user-images.githubusercontent.com/100347457/216971624-9f456f4d-a8dc-44fb-8047-11cc882537ed.png" width="100%"/></p>

<p align="center">
<img alt="" src="https://user-images.githubusercontent.com/100347457/216972254-10d0c9fd-34eb-43f4-94dd-61e882bcc364.png" width="49%"/>
<img alt="" src="https://user-images.githubusercontent.com/100347457/216972251-41a413e5-89f7-4c8c-b41b-a291470cba59.png" width="49%"/>
</p>

<div align="center">

Application translated in: 🇬🇧 - 🇮🇹 <br>
[More languages will be supported in the upcoming releases](https://github.com/GyulyVGC/sniffnet/issues/60)

</div>

<hr>


## Installation

You can install Sniffnet in one of the following ways:

<details>

  <summary>from Crates.io&ensp;<img alt="" src="https://img.shields.io/crates/d/sniffnet?color=success&label=downloads&logo=rust"/></summary>

  Follow this method only if you have [Rust installed](https://www.rust-lang.org/tools/install) on your machine. <br>
  In this case, the application binary can be installed with: 

```sh
cargo install sniffnet
```
    
</details>


<details>

  <summary>from GitHub releases&ensp;<img alt="" src="https://img.shields.io/github/downloads/gyulyvgc/sniffnet/total?color=success&logo=github"/></summary>

  You can install Sniffnet through the installers available in the [latest release](https://github.com/GyulyVGC/sniffnet/releases). <br>
  Choose from a Windows installer, a macOS disk image, or a DEB package (depending on your operating system). <br>
  Here for your convenience you can find the direct link to the downloads:
  - [Windows](https://github.com/GyulyVGC/sniffnet/releases/download/v1.1.0/Sniffnet_Windows.msi) (7.3 MB)
  - [macOS](https://github.com/GyulyVGC/sniffnet/releases/download/v1.1.0/Sniffnet_MacOS.dmg) (6.5 MB)
  - [Linux .deb](https://github.com/GyulyVGC/sniffnet/releases/download/v1.1.0/Sniffnet_Linux.deb) (5.2 MB)

    
</details>


<details>

  <summary>from Homebrew&ensp;<img alt="" src="https://img.shields.io/homebrew/installs/dm/sniffnet?color=success&logo=homebrew"/></summary>

  You can install Sniffnet's Homebrew package with:
  
  ```sh
brew install sniffnet
```
    
</details>


## Required dependencies

Depending on your operating system, you may need to install some dependencies to run Sniffnet:

<details>

  <summary>Windows dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474292-d84f2a96-f445-40ac-8930-9d0f00a3c2bb.png" width="35px"/></summary>

  In order to correctly run Sniffnet on Windows systems you need to:

  - Install [Npcap](https://npcap.com/#download).

  - Download the [Npcap SDK](https://npcap.com/#download).

  - Add the SDK's ```/Lib``` or ```/Lib/x64``` folder to your ```LIB``` environment variable.
    
</details>


<details>

  <summary>Linux dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474239-c48d37af-d4c1-4a94-9207-0d46c6d75f1f.png" width="35px"/></summary>
 
  In order to correctly run Sniffnet on Linux systems, install the libraries and header files for the libpcap library: 
  
```sh
sudo apt-get install libpcap-dev
```

  Note that if you are not running as root, you need to set capabilities to inspect a network adapter: 
  
```sh
sudo setcap cap_net_raw,cap_net_admin=eip <your/Sniffnet/executable/path>
```

Most Linux system also need this dependency (required to build the library used to play sounds):

```sh
sudo apt-get install libasound2-dev
```

Depending on your Linux environment you may also need `libfontconfig`:

```sh
sudo apt-get install libfontconfig libfontconfig1-dev
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
- 🔉 set custom notifications to inform you when defined network events occur (data rate exceeded a specified threshold, or new data have been exchanged from your favorite connections)
- ⭐ view most relevant connections in real time (most recent, most packets, most bytes, favorites)
- 🌍 get information about the country of the remote network addresses (IP Geolocation)
- 🎨 choose the style that fits you the most from 4 different available themes 
- 📁 save complete textual report with detailed information for each connection:
  * source and destination IP addresses
  * source and destination ports
  * carried protocols
  * amount of exchanged packets and bytes
  * initial and final timestamp of information exchange
- ... and more!
  

## IP Geolocation

<details>

  <summary>See details</summary>

  <br>

  Geolocation refers to the remote IP address of the connection, and it's performed against a [MMDB file](https://maxmind.github.io/MaxMind-DB/):

  > The MMDB (MaxMind database) format has been developed especially for IP lookup. It is optimized to perform lookups on data indexed by IP network ranges quickly and efficiently. If you want the best performance on your IP lookups for use in a production environment, you should use the MMDB format files.
  
  This format potentially allows Sniffnet to execute different hundreds of IP lookups in a matter of a few milliseconds.

  Sometimes it is not possible to determine the location of an IP address; this is most likely due to the address being a private IP address.

</details>

## Supported application layer protocols

<details>

  <summary>See details</summary>
  
  <br>
  
  Please, note that application layer protocols are just inferred from the transport port numbers.
  
  <br>
  
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


## Troubleshooting

<details>

  <summary>See details</summary>

### Missing dependencies

Most of the errors that can occur are likely due to your system missing required `pcap` dependencies,
necessary to correctly analyze a network adapter. <br>
Check the [required dependencies](#required-dependencies) section for instructions on how to proceed.

For a Windows reference, you can check issue [#1](https://github.com/GyulyVGC/sniffnet/issues/1).

Note that most Linux system also need this dependency (required to build the library used to play sounds):

```sh
sudo apt-get install libasound2-dev
```

Some Linux systems also need `libfontconfig`, see issue [#18](https://github.com/GyulyVGC/sniffnet/issues/18) for a reference.

### Installers incompatibilities

If you have problems after having installed Sniffnet through the provided installers,
it could be due to your OS not being compatible with the pre-built binaries I generated for you. <br>
Reach me out, and I'll try to generate an installer for your specific operating system.

### ***In any case don't hesitate to [open an issue](https://github.com/GyulyVGC/sniffnet/issues), and I will do my best to help you!***

</details>

<!---
## Contribute

Do you want to improve Sniffnet? Check [here](https://github.com/GyulyVGC/sniffnet/blob/main/CONTRIBUTING.md) 

Sniffnet is also open to design contributions: <br>
[![contribute.design](https://contribute.design/api/shield/GyulyVGC/sniffnet)](https://contribute.design/GyulyVGC/sniffnet)
--->

## Acknowledgements

A big shout-out to [all the contributors](https://github.com/GyulyVGC/sniffnet/graphs/contributors) of Sniffnet ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<div align="center">

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="10%"><a href="https://github.com/Crirock"><img src="https://avatars.githubusercontent.com/u/101053125?v=4?s=60" width="60px;" alt="Cristiano"/><br /><sub><b>Cristiano</b></sub></a><br /><a href="https://github.com/GyulyVGC/sniffnet/commits?author=Crirock" title="Code">💻</a> <a href="#ideas-Crirock" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="10%"><a href="https://github.com/acolombier"><img src="https://avatars.githubusercontent.com/u/7086688?v=4?s=60" width="60px;" alt="Antoine Colombier"/><br /><sub><b>Antoine Colombier</b></sub></a><br /><a href="https://github.com/GyulyVGC/sniffnet/commits?author=acolombier" title="Tests">⚠️</a> <a href="#translation-acolombier" title="Translation">🌍</a></td>
      <td align="center" valign="top" width="10%"><a href="https://github.com/colin99d"><img src="https://avatars.githubusercontent.com/u/72827203?v=4?s=60" width="60px;" alt="Colin Delahunty"/><br /><sub><b>Colin Delahunty</b></sub></a><br /><a href="https://github.com/GyulyVGC/sniffnet/commits?author=colin99d" title="Tests">⚠️</a></td>
      <td align="center" valign="top" width="10%"><a href="https://github.com/AmadeusGraves"><img src="https://avatars.githubusercontent.com/u/18572939?v=4?s=60" width="60px;" alt="AmadeusGraves"/><br /><sub><b>AmadeusGraves</b></sub></a><br /><a href="#translation-AmadeusGraves" title="Translation">🌍</a></td>
      <td align="center" valign="top" width="10%"><a href="https://github.com/Charpy"><img src="https://avatars.githubusercontent.com/u/4827568?v=4?s=60" width="60px;" alt="Charpy"/><br /><sub><b>Charpy</b></sub></a><br /><a href="#translation-Charpy" title="Translation">🌍</a></td>
      <td align="center" valign="top" width="10%"><a href="https://github.com/Kiruyuto"><img src="https://avatars.githubusercontent.com/u/80201134?v=4?s=60" width="60px;" alt="Hubert"/><br /><sub><b>Hubert</b></sub></a><br /><a href="#translation-Kiruyuto" title="Translation">🌍</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

</div>

The graphical user interface has been realized with [iced](https://github.com/iced-rs/iced), a cross-platform GUI library for Rust focused on simplicity and type-safety

<p align="center"><a href="https://github.com/iced-rs/iced"><img alt="" src="https://user-images.githubusercontent.com/100347457/219339409-0a44722b-416d-410b-93a4-8b0e84c0031d.svg" width="50%"/></a></p>

Last but not least, thanks to [every single stargazer](https://github.com/GyulyVGC/sniffnet/stargazers): all forms of support made it possible to keep improving Sniffnet!

<p align="center"><a href="https://github.com/GyulyVGC/sniffnet/stargazers"><img alt="" src="https://reporoster.com/stars/dark/GyulyVGC/sniffnet"/></a></p>
