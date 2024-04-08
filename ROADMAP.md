# Roadmap

This file provides an overview of the direction this project is heading.

Even if I no longer have the luck of working on Sniffnet full-time (as described in [#329](https://github.com/GyulyVGC/sniffnet/discussions/329)),
I'll try all my best to make the application grow even further by targeting the following features, improvements, and fixes. <br>

Sections are sorted by relevance. <br>
Elements inside a section are sorted by expected release date. <br>
Known open problems are reported at the bottom of this file.

## Major features
  
| Feature                                     | Release  | 
|---------------------------------------------|----------|
| IP geolocation                              | ✅ (v1.1) |
| Custom notifications                        | ✅ (v1.1) |
| Host names and ASN availability             | ✅ (v1.2) |
| Full connections report in a dedicated page | ✅ (v1.2) |
| Thumbnail mode                              | ✅ (v1.3) |
| Extended upper layer services recognition   | ✅ (v1.3) |
| ICMP support                                | ✅ (v1.3) |
| PCAP file export                            | ✅ (v1.3) |
| PCAP file import                            | ❓(TBD)   |
| IP addresses' reputation                    | ❓(TBD)   |
| Malicious traffic detection                 | ❓(TBD)   |
| PIDs identification                         | ❓(TBD)   |
| Packets' payload inspection                 | ❓(TBD)   |
| Firewall capabilities                       | ❓(TBD)   |
| Sniffnet agent to monitor a remote host     | ❓(TBD)   |
| Web interface                               | ❓(TBD)   |

## Other features
  
| Feature                                            | Release    | 
|----------------------------------------------------|------------|
| Settings page: notifications, style, language      | ✅ (v1.1.0) |
| Keyboard shortcuts                                 | ✅ (v1.1.2) |
| Notify user when new release is available          | ✅ (v1.1.3) |
| Styles based on color gradients                    | ✅ (v1.2.2) |
| Custom TOML themes support                         | ✅ (v1.3.0) |
| Filter traffic by port number                      | ✅ (v1.3.0) |
| Advanced settings: UI scale factor and MMDBs paths | ✅ (v1.3.0) |
| Proper table structure for connections list        | ✅ (v1.3.0) |
| Extended documentation (Wiki)                      | ✅ (v1.3.0) |
| Support more link types                            | ✅ (v1.3.0) |
| Filter inputs autocompletion                       | ❓(TBD)     |
| Details about unassigned IPs                       | ❓(TBD)     |
| Additional details about each notification event   | ❓(TBD)     |

## Known problems

Solved problems will be removed. <br>
This section only contains currently open problems.

- Complete host report availability
  - only the 30 top hosts are shown in GUI at the moment
- Notifications startegy should be changed
  - too many notifications are delivered, related to similar events, in a short amount of time
- Improve packages
  - remove previous version of Sniffnet in Windows
  - solve problems related to osascript on macOS
  - see [#252](https://github.com/GyulyVGC/sniffnet/issues/252) for the complete list of packaging-related problems
- Text inputs for notification settings immediately change the threshold value while the user is typing
- Scrollers change position when an overlay is opened
- The selected adapter may not be in the visible portion of scrollbar when opening the app
- `cosmic-text` doesn't display some text correctly centered
- Charts' grids are missing when the renderer used is `tiny-skia`
