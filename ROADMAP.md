# Roadmap

This file provides an overview of the direction this project is heading.

As described in [#329](https://github.com/GyulyVGC/sniffnet/discussions/329), I no longer have the possibility of working on Sniffnet full-time, so expect updates to come at a slower pace with respect to the past. <br>
I'll keep maintaining Sniffnet in my spare time and I'll try my best to target all the following points.

Sections are sorted by relevance. <br>
Elements inside a section are sorted by expected release date. <br>
Known open problems are reported at the bottom of this file.

## Major features
  
Feature | Release | 
-|-|
IP geolocation | ✅ (v1.1)
Custom notifications | ✅ (v1.1)
Host names and ASN availability | ✅ (v1.2)
Full connections report in a dedicated page | ✅ (v1.2)
Read and write of PCAP files | 🔜 (v1.3)
ICMP support | 🔜 (v1.3)
Packets' payload inspection | ❓(TBD)
IP addresses' reputation | ❓(TBD)
Malicious traffic detection | ❓(TBD)
PIDs identification | ❓(TBD)
Sniffnet agent to monitor a remote host | ❓(TBD)

## Other features
  
Feature | Release | 
-|-|
Settings page: notifications, style, language | ✅ (v1.1.0)
Keyboard shortcuts | ✅ (v1.1.2)
Notify user when new release is available | ✅ (v1.1.3)
Styles based on color gradients | 🔜 (v1.2.2)
Filter traffic by port number | 🔜 (v1.3)
Custom UI scale factor | 🔜 (v1.3)
Custom path for MMDBs  | ❓(TBD)
Proper table structure for connections list | ❓(TBD)
Filter inputs autocompletion | ❓(TBD)
Details about unassigned IPs | ❓(TBD)
Custom TOML themes support | ❓(TBD)
Additional details about each notification event | ❓(TBD)

## Known problems

Solved problems will be removed. <br>
This section only contains currently open problems.

- Complete host report availability
  - only the 30 top hosts are shown in GUI at the moment
- Notifications startegy should be changed
  - too many notifications are delivered, related to similar events, in a short amount of time
- Improve output report update strategy
  - currently the output is generated once per second
- Using the default renderer in some environments can cause some layout problems:
  - flags are not renderer correctly (they appear black)
  - the UI screen glitches
  - see [this section](https://github.com/GyulyVGC/sniffnet#troubleshooting) to solve
- Improve packages
  - remove previous version of Sniffnet in Windows
  - solve problems related to osascript on macOS
  - see [#252](https://github.com/GyulyVGC/sniffnet/issues/252) for the complete list of packaging-related problems
- Text inputs for notification settings immediately change the threshold value while the user is typing
- Scrollers change position when an overlay is opened
- The selected adapter may not be in the visible portion of scrollbar when opening the app
- `cosmic-text` doesn't display some text correctly centered
- Charts' grids are missing when the renderer used is `tiny-skia`
