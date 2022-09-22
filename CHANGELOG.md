# Change Log
All Sniffnet releases with the relative changes are documented in this file.
 
## [0.1.0] - 2022-08-17
  
- Sniffnet first release


## [0.1.1] - 2022-08-17
  
- Fixed README errors


## [0.1.2] - 2022-08-18
  
- Added video tutorial about the application


## [0.2.0] - 2022-08-24
  
- Added command line option `--app` to filter application layer protocols
- Added feature to recognize local vs remote addresses 
- Added function to parse IPv6 addresses
- Fixed secondary threads panics
- Changed the way application layer protocols are retrieved
- Improved textual report format


## [0.2.1] - 2022-08-26

- Removed img folder and uploaded pictures on cloud


## [0.3.0] - 2022-08-29

- Added global statistics: number of [address:port] pairs and sniffed packets
- Added statistics on the number of packets for each application layer protocol
- Fixed application layer protocols filtering


## [0.3.1] - 2022-08-31

- Added devices description when application is launched with the `-d` option
- Intruduced feature to measure write timings and added a BufWriter to improve write performance
- Fixed standard output colors for Windows systems


## [0.3.2] - 2022-09-07

- Changed output report structure: each element now corresponds to a couple of network [address:port]
- When application is resumed after pause, the buffer containing packets is reinitialized


## [0.4.0] - 2022-09-11

- Added feature to produce a graphical report with the number of packets per second and the number of bits per seconds, incoming and outgoing
- Added multicast addresses recognition
- Reports are not updated if the application is paused
