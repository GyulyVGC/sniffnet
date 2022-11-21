# Change Log
All Sniffnet releases with the relative changes are documented in this file.


## [1.0.0] - 2022-11-21

- The application is no longer just a command line interface: Sniffnet has now a whole graphical user interface!
  * Charts and traffic statistics are now constantly updated and shown interactively in the GUI
  * Users don't have to worry about command line options anymore: it is now possible to comfortably specify adapters and filters through the GUI
  * Sniffnet is now more accessible, available in real-time, easy to use and aesthetically pleasing thanks to its new interface
- In order to reach out as many people as possible, I created [installers](https://github.com/GyulyVGC/sniffnet/releases) for Windows, macOS and Linux, to make it easier to install Sniffnet for those that still doesn't have Rust on their machines


## [0.5.0] - 2022-10-02

- Optimized textual report updates: only changed entries are rewritten (file `report.txt`)
- Textual report elements are now ordered by timestamp instead of number of packets
- Report header with statistics is now written on a separate textual file (file `statistics.txt`)
- Removed command line option `--verbose` because considered redundant
- Removed command line option `--minimum-packets` because not meaningful anymore


## [0.4.1] - 2022-09-27

- Changed the default textual report representation
- Added command line option `-v` to set the textual report representation to the former one (verbose mode)
- Sniffnet now also considers the transport layer protocol to define textual report elements (now defined by the network 5-tuple)


## [0.4.0] - 2022-09-11

- Added feature to produce a graphical report with the number of packets per second and the number of bits per seconds, incoming and outgoing
- Added multicast addresses recognition
- Reports are not updated if the application is paused


## [0.3.2] - 2022-09-07

- Changed output report structure: each element now corresponds to a couple of network [address:port]
- When application is resumed after pause, the buffer containing packets is reinitialized


## [0.3.1] - 2022-08-31

- Added devices' description when application is launched with the `-d` option
- Introduced feature to measure write timings and added a BufWriter to improve write performance
- Fixed standard output colors for Windows systems


## [0.3.0] - 2022-08-29

- Added global statistics: number of [address:port] pairs and sniffed packets
- Added statistics on the number of packets for each application layer protocol
- Fixed application layer protocols filtering


## [0.2.1] - 2022-08-26

- Removed img folder and uploaded pictures on cloud


## [0.2.0] - 2022-08-24
  
- Added command line option `--app` to filter application layer protocols
- Added feature to recognize local vs remote addresses 
- Added function to parse IPv6 addresses
- Fixed secondary threads panics
- Changed the way application layer protocols are retrieved
- Improved textual report format


## [0.1.2] - 2022-08-18
  
- Added video tutorial about the application


## [0.1.1] - 2022-08-17
  
- Fixed README errors


## [0.1.0] - 2022-08-17
  
- Sniffnet first release
