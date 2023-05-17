# Change Log

All Sniffnet releases with the relative changes are documented in this file.


## [1.2.0] - 2023-05-18

- Introduced host-based analysis: instead of just showing IP addresses, now host names and network providers are available for a quicker and more meaningful traffic interpretation 
  * Added rDNS (reverse DNS) lookups to find out network host names
  * Added ASN (Autonomous System name and number) lookups to find out the entity managing a given IP address (fixes [#62](https://github.com/GyulyVGC/sniffnet/issues/62))
- Individual connections identified by IP addresses remain available and can now be filtered and further inspected through a simple click
- Support for identification of addresses in the local network
- Support for data link layer MAC addresses
- Full support for broadcast traffic recognition (added directed broadcast identification)
- Added dropped packets number (fixes [#135](https://github.com/GyulyVGC/sniffnet/issues/135))
- Changed favorites management: instead of referring to single IP addresses, favorites are now related to network hosts
- Added Greek translation 🇬🇷 ([#160](https://github.com/GyulyVGC/sniffnet/pull/160))
- Added Persian translation 🇮🇷 ([#158](https://github.com/GyulyVGC/sniffnet/pull/158))
- Do not open terminal window when starting the application on Windows (fixes [#85](https://github.com/GyulyVGC/sniffnet/issues/85))
- Do not open terminal window when starting the application on macOS
- Changed macOS application icon to be consistent with standard icons dimension (fixes [#177](https://github.com/GyulyVGC/sniffnet/issues/177))
- Made available RPM package for Linux and automated packaging process for Windows, macOS, and Linux ([#180](https://github.com/GyulyVGC/sniffnet/pull/180) - fixes [#20](https://github.com/GyulyVGC/sniffnet/issues/20))
- Keep the active addresses of the selected network adapter up to date during analysis
- Changed shortcut to interrupt analysis from `backspace` to `ctrl+backspace`
- Images have been replaced with SVGs
- Added unit tests for `chart` and started unit tests for `gui` modules ([#132](https://github.com/GyulyVGC/sniffnet/pull/132))
- Fixed problem that let users switch page pressing the tab key even if no packets were received


## [1.1.4] - 2023-04-18

- Added new translations of the GUI:
  * Portuguese 🇵🇹 ([#134](https://github.com/GyulyVGC/sniffnet/pull/134))
  * Russian 🇷🇺 ([#151](https://github.com/GyulyVGC/sniffnet/pull/151))
  * Korean 🇰🇷 ([#128](https://github.com/GyulyVGC/sniffnet/pull/128))
  * Turkish 🇹🇷 ([#139](https://github.com/GyulyVGC/sniffnet/pull/139))
  * ...the total number of supported languages is now 13 🎉
- Changed adapter buttons format and improved volume slider layout (see [#119](https://github.com/GyulyVGC/sniffnet/issues/119) for more details or to give me further suggestions)
- Scrollbars are now highlighted when hovering on the respective scrollable area
- Set up `iced_glow` feature on branch [`glow-renderer`](https://github.com/GyulyVGC/sniffnet/tree/glow-renderer) to overcome unsupported graphics ([#155](https://github.com/GyulyVGC/sniffnet/pull/155))
- Modified `dependabot` configuration to update GitHub Actions as needed ([#141](https://github.com/GyulyVGC/sniffnet/pull/141))
- Fixed problem causing a crash on macOS when starting Sniffnet's Homebrew package or building from source in release mode ([#109](https://github.com/GyulyVGC/sniffnet/issues/109) - [#137](https://github.com/GyulyVGC/sniffnet/issues/137))


## [1.1.3] - 2023-04-04

- Added Romanian translation 🇷🇴 ([#113](https://github.com/GyulyVGC/sniffnet/pull/113))
- Added feature to warn you when a newer version of Sniffnet is available on GitHub 🆕 ([#118](https://github.com/GyulyVGC/sniffnet/pull/118))
- Added badge on tab bar to show unread notifications count 🔉
- Introduction of `lazy` widgets to improve the application efficiency ([#122](https://github.com/GyulyVGC/sniffnet/pull/122))
- Aesthetic improvements to create a more modern and minimal UI ([#119](https://github.com/GyulyVGC/sniffnet/issues/119))
- Changed keyboard shortcut to open settings from `ctrl+S` to `ctrl+,`, as suggested in [#97](https://github.com/GyulyVGC/sniffnet/issues/97)
- Fixed problem that was causing a switch to the initial page when back button was pressed with settings opened on running page and with no packets received
- Fixed problem that was causing application logo to be partially hidden when resizing the window to a lower dimension
- Show `-` option in app protocol picklist only when a filter is active
- Refactored and cleaned code modules ([#123](https://github.com/GyulyVGC/sniffnet/pull/123))
- Fixed header alignment


## [1.1.2] - 2023-03-18

- Added new translations of the GUI, bringing the total number of supported languages to 8!
  * German 🇩🇪 ([#87](https://github.com/GyulyVGC/sniffnet/pull/87))
  * Simplified Chinese 🇨🇳 ([#89](https://github.com/GyulyVGC/sniffnet/pull/89) - [#93](https://github.com/GyulyVGC/sniffnet/pull/93))
  * Ukrainian 🇺🇦 ([#94](https://github.com/GyulyVGC/sniffnet/pull/94))
- Added keyboard shortcuts to make the whole experience more enjoyable and efficient:
  check out issue [#97](https://github.com/GyulyVGC/sniffnet/issues/97) to see all the available hotkeys or to suggest new ones!
- Changed GUI font to `sarasa-gothic-mono` to support the introduction of Simplified Chinese language
- Minor improvements to Overview page proportions and paddings


## [1.1.1] - 2023-02-25

- Added new translations of the GUI!
  * French 🇫🇷 ([#64](https://github.com/GyulyVGC/sniffnet/pull/64) - [#67](https://github.com/GyulyVGC/sniffnet/pull/67))
  * Spanish 🇪🇦 ([#70](https://github.com/GyulyVGC/sniffnet/pull/70))
  * Polish 🇵🇱 ([#78](https://github.com/GyulyVGC/sniffnet/pull/78))
- The last successfully sniffed network adapter is now remembered on application closure, so that users don't have to manually select it again when restarting Sniffnet (implementing a feature requested in [#77](https://github.com/GyulyVGC/sniffnet/issues/77))
- Implemented possibility to quit the application pressing crtl+Q keys, as requested in [#68](https://github.com/GyulyVGC/sniffnet/issues/68)
- The last opened settings page is now remembered within a given session
- Fixed bug that caused settings configuration not to be permanently saved across different sessions when closing settings from the 'x' button in the top right corner (fixes [#77](https://github.com/GyulyVGC/sniffnet/issues/77))
- Textual report is now saved in a fixed directory, instead of using the directory where the execution was started. The output is now saved in the same folder containing configuration files storing Sniffnet settings. The directory is automatically chosen by [confy](https://docs.rs/confy/0.5.1/confy/) depending on your architecture, and can be seen hovering on the "Open full report" button. (fixes [#51](https://github.com/GyulyVGC/sniffnet/issues/51))
- When multiple favorite connections are featured per time interval, now it's possible to receive more than one favorite notification referred to the same timestamp
- Fixed problem that was causing the Application Protocol picklist placeholder not being translated


## [1.1.0] - 2023-02-07

- Added Custom Notifications to inform the user when defined network events occur:
  * data intensity exceeded a defined packets per second rate
  * data intensity exceeded a defined bytes per second rate
  * new data are exchanged from one of the favorite connections
- Added Settings pages to configure the state of the application (persistently stored in a configuration file):
  * customise notifications
  * choose between 4 different application styles
  * set the application language (this release introduces the Italian language 🇮🇹, and more languages will be supported soon)
- Added Geolocation of the remote IP addresses (consult the README for more information)
- Implemented the possibility of marking a group of connections as favorites and added favorites view to the report
- Added modal to ask the user for confirmation before leaving the current analysis
- Added Tooltips to help the user better understand the function of some buttons
- Partially implemented support for broadcast IP addresses (still missing IPv4 directed broadcast)
- The application window is now maximized after start
- All the GUI text fonts have been replaced with 'Inconsolata'
- Fixed issue [#48](https://github.com/GyulyVGC/sniffnet/issues/48) adding a horizontal scrollable to the report view



## [1.0.1] - 2022-11-30

- Substituted command `open` with command `xdg-open` on Linux systems to solve the problem described in issues [#13](https://github.com/GyulyVGC/sniffnet/issues/13) and [#23](https://github.com/GyulyVGC/sniffnet/issues/23)
- Introduced a constraint on minimum window height to avoid problem described in issue [#12](https://github.com/GyulyVGC/sniffnet/issues/12)
- Added some tests on `AppProtocol` and improved GitHub workflows


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
