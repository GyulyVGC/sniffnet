<p align="center"><a href="https://github.com/GyulyVGC/sniffnet"><img alt="Sniffnet" src="https://user-images.githubusercontent.com/100347457/205967236-e086d1df-a266-4127-8e73-c2cbb889c007.png" width="100%"/></a></p>


<hr>

<p align="center"> Application to comfortably monitor your network traffic </p>
<p align="center"> Multithreaded, cross-platform, reliable </p>

<p align="center">
<a href="https://crates.io/crates/sniffnet"><img alt="" src="https://img.shields.io/crates/v/sniffnet?color=orange&logo=rust"/></a>
&nbsp;
<a href="https://crates.io/crates/sniffnet"><img alt="" src="https://img.shields.io/crates/d/sniffnet?color=orange&label=crate%20downloads&logo=rust"/></a>
</p>

<p align="center">
<a href="https://github.com/GyulyVGC/sniffnet/stargazers"><img alt="" src="https://img.shields.io/github/stars/gyulyvgc/sniffnet?logo=github"/></a>
&nbsp;
<a href="https://github.com/GyulyVGC/sniffnet/releases"><img alt="" src="https://img.shields.io/github/downloads/gyulyvgc/sniffnet/total?color=blue&logo=github"/></a>
</p>


<hr>

<p align="center"><img alt="" src="https://user-images.githubusercontent.com/100347457/205967647-ef77cf89-20b0-49e3-a10d-0d4fe1da7752.gif" width="85%"/></p>

<div align="center">
<p>Sniffnet is a simple yet insightful application to let you have a glance into your network traffic 
in a straightforward and appealing way </p>
</div>

<hr>


## Install and Run

If you have [Rust installed](https://www.rust-lang.org/tools/install) on your machine, the application binary can be installed with: 
```sh
cargo install sniffnet
```

Otherwise, you can install Sniffnet through the installers available in the [latest release](https://github.com/GyulyVGC/sniffnet/releases).


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

- choose a network adapter to inspect
<p align="center"><img alt="" src="https://user-images.githubusercontent.com/100347457/205967942-1ee5d1b2-222e-46ee-bad7-e9aa63492628.png" width="50%"/></p>

- select filters to apply to the observed traffic
<p align="center"><img alt="" src="https://user-images.githubusercontent.com/100347457/205968099-a68f32c4-c077-4f82-be2c-1cf88ce5522d.png" width="60%"/></p>
  
- view real-time charts about traffic intensity (bytes and packets per second, incoming and outgoing)
<p align="center"><img alt="" src="https://user-images.githubusercontent.com/100347457/205968195-f979f1b4-b737-4ece-8433-cf539d140eb4.png" width="80%"/></p>

- view overall statistics about the filtered traffic
<p align="center"><img alt="" src="https://user-images.githubusercontent.com/100347457/205968251-b331a2b7-14df-45ab-aafd-20d93d9156bf.png" width="50%"/></p>

- view most relevant connections (most recent, most packets, most bytes)
<p align="center"><img alt="" src="https://user-images.githubusercontent.com/100347457/205967785-ff98bc0a-f3e8-44ad-bafe-4b9a46344f74.png" width="95%"/></p>

- save complete textual report with detailed information for each connection:
  * source and destination IP addresses
  * source and destination ports
  * carried protocols
  * amount of exchanged packets and bytes
  * initial and final timestamp of information exchange
  

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
necessary to correctly analyze a network adapter. 

Check the [Install and Run](#install-and-run) section for instructions on how to proceed.

For a Windows reference, you can check issue [#1](https://github.com/GyulyVGC/sniffnet/issues/1).

Some Linux systems also need `libfontconfig`, see issue [#18](https://github.com/GyulyVGC/sniffnet/issues/18) for a reference.

### Installers incompatibilities

If you have problems after having installed Sniffnet through the provided installers,
it could be due to your OS not being compatible with the pre-built binaries I generated for you.

Reach me out, and I'll try to generate an installer for your specific operating system.

### ***In any case don't hesitate to [open an issue](https://github.com/GyulyVGC/sniffnet/issues), and I will do my best to help you!***

</details>


## Contribute

Do you want to improve Sniffnet? Check [here](https://github.com/GyulyVGC/sniffnet/blob/main/CONTRIBUTING.md) 

Sniffnet is also open to design contributions: 

[![contribute.design](https://contribute.design/api/shield/GyulyVGC/sniffnet)](https://contribute.design/GyulyVGC/sniffnet)


## Stargazers

<a href="https://github.com/GyulyVGC/sniffnet/stargazers"><img alt="" src="https://reporoster.com/stars/dark/GyulyVGC/sniffnet"/></a>
