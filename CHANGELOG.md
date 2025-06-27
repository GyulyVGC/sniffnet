# Changelog

All Sniffnet releases with the relative changes are documented in this file.

## [1.4.0] - 2025-06-27
- Import PCAP files ([#795](https://github.com/GyulyVGC/sniffnet/pull/795) — fixes [#283](https://github.com/GyulyVGC/sniffnet/issues/283))
- Donut chart reporting overall traffic statistics ([#756](https://github.com/GyulyVGC/sniffnet/pull/756) — fixes [#687](https://github.com/GyulyVGC/sniffnet/issues/687))
- Notifications: more details and other improvements ([#830](https://github.com/GyulyVGC/sniffnet/pull/830) — fixes [#637](https://github.com/GyulyVGC/sniffnet/issues/637))
- Added support for ARP protocol ([#759](https://github.com/GyulyVGC/sniffnet/pull/759) — fixes [#680](https://github.com/GyulyVGC/sniffnet/issues/680))
- Identify and tag unassigned/reserved "bogon" IP addresses ([#678](https://github.com/GyulyVGC/sniffnet/pull/678) — fixes [#209](https://github.com/GyulyVGC/sniffnet/issues/209))
- Show data agglomerates in _Inspect_ page table ([#684](https://github.com/GyulyVGC/sniffnet/pull/684) — fixes [#601](https://github.com/GyulyVGC/sniffnet/issues/601))
- Added Traditional Chinese (Taiwan) translation 🇹🇼 ([#774](https://github.com/GyulyVGC/sniffnet/pull/774))
- Added Indonesian translation 🇮🇩 ([#611](https://github.com/GyulyVGC/sniffnet/pull/611))
- A Docker image of Sniffnet is now available ([#735](https://github.com/GyulyVGC/sniffnet/pull/735))
- Updated some of the existing translations to v1.3: 
  - Portuguese ([#690](https://github.com/GyulyVGC/sniffnet/pull/690))
  - Ukrainian ([#692](https://github.com/GyulyVGC/sniffnet/pull/692))
  - Spanish ([#805](https://github.com/GyulyVGC/sniffnet/pull/805))
- Do not apply new notification thresholds while user is typing them ([#777](https://github.com/GyulyVGC/sniffnet/pull/777) — fixes [#658](https://github.com/GyulyVGC/sniffnet/issues/658))
- Show more information when domain name is short ([#720](https://github.com/GyulyVGC/sniffnet/pull/720) — fixes [#696](https://github.com/GyulyVGC/sniffnet/issues/696))
- Added new themes _A11y (Night)_ and _A11y (Day)_ based on palettes optimized for Accessibility ([#785](https://github.com/GyulyVGC/sniffnet/pull/785) — fixes [#786](https://github.com/GyulyVGC/sniffnet/issues/786))
- Avoid directory traversal when selecting file name for PCAP exports ([#776](https://github.com/GyulyVGC/sniffnet/pull/776) — fixes [#767](https://github.com/GyulyVGC/sniffnet/issues/767))
- Add icon to window title bar ([#719](https://github.com/GyulyVGC/sniffnet/pull/719) — fixes [#715](https://github.com/GyulyVGC/sniffnet/issues/715))
- Update footer buttons and links ([#755](https://github.com/GyulyVGC/sniffnet/pull/755) — fixes [#553](https://github.com/GyulyVGC/sniffnet/issues/553))
- Handle errors to reduce the number of possible crash occurrences ([#784](https://github.com/GyulyVGC/sniffnet/pull/784))
- Use asynchronous channels to update app state from backend ([#806](https://github.com/GyulyVGC/sniffnet/pull/806))
- Fix _crates.io_ package for Windows ([#718](https://github.com/GyulyVGC/sniffnet/pull/718) — fixes [#681](https://github.com/GyulyVGC/sniffnet/issues/681))
- Fix crash when inserting characters longer than one byte in the text input for byte threshold notification setting ([#747](https://github.com/GyulyVGC/sniffnet/pull/747) — fixes [#744](https://github.com/GyulyVGC/sniffnet/issues/744))
- Remove pre-uninstall script on Linux (fixes [#644](https://github.com/GyulyVGC/sniffnet/issues/644))
- Fix typo in Russian translation (fixes [#730](https://github.com/GyulyVGC/sniffnet/issues/730))
- Minor fix to service determination algorithm in case of multicast and broadcast traffic

## [1.3.2] - 2025-01-06
- Dropdown menus for network host filters ([#659](https://github.com/GyulyVGC/sniffnet/pull/659) — fixes [#354](https://github.com/GyulyVGC/sniffnet/issues/354))
- Added CLI argument `--adapter [<NAME>]` to allow immediately starting the capture from a given network interface ([#643](https://github.com/GyulyVGC/sniffnet/pull/643) — fixes [#636](https://github.com/GyulyVGC/sniffnet/issues/636))
- Added Vietnamese translation 🇻🇳 ([#577](https://github.com/GyulyVGC/sniffnet/pull/577))
- Ask for quit confirmation before stopping an ongoing analysis ([#652](https://github.com/GyulyVGC/sniffnet/pull/652) — fixes [#570](https://github.com/GyulyVGC/sniffnet/issues/570))
- Redirect `stderr` and `stdout` to file on Windows release builds ([#645](https://github.com/GyulyVGC/sniffnet/pull/645) — fixes [#578](https://github.com/GyulyVGC/sniffnet/issues/578))
- Added Wiki page describing [command line arguments](https://github.com/GyulyVGC/sniffnet/wiki/Command-line-arguments) (fixes [#642](https://github.com/GyulyVGC/sniffnet/issues/642))
- Updated some of the existing translations to v1.3: 
  - Chinese ([#575](https://github.com/GyulyVGC/sniffnet/pull/575))
  - Korean ([#604](https://github.com/GyulyVGC/sniffnet/pull/604))
  - Turkish ([#608](https://github.com/GyulyVGC/sniffnet/pull/608))
- Improve time values on the horizontal axis of the chart ([#641](https://github.com/GyulyVGC/sniffnet/pull/641) — fixes [#619](https://github.com/GyulyVGC/sniffnet/issues/619))
- Migrate to Iced 0.13 ([#618](https://github.com/GyulyVGC/sniffnet/pull/618))
- Added support for Linux `loongarch64` (fixes [#592](https://github.com/GyulyVGC/sniffnet/issues/592))
- Fix typos in German translation ([#660](https://github.com/GyulyVGC/sniffnet/pull/660))

## [1.3.1] - 2024-07-22
- Thumbnail mode improvements ([#512](https://github.com/GyulyVGC/sniffnet/pull/512))
- Support IPinfo ASN and Country databases ([#558](https://github.com/GyulyVGC/sniffnet/pull/558) — fixes [#533](https://github.com/GyulyVGC/sniffnet/issues/533))
- Added keyboard shortcuts to change zoom level (fixes [#554](https://github.com/GyulyVGC/sniffnet/issues/554))
- Increased the range of selectable zoom values (fixes [#542](https://github.com/GyulyVGC/sniffnet/issues/542))
- Updated some of the existing translations to v1.3: 
  - French ([#494](https://github.com/GyulyVGC/sniffnet/pull/494))
  - German ([#495](https://github.com/GyulyVGC/sniffnet/pull/495))
  - Russian ([#496](https://github.com/GyulyVGC/sniffnet/pull/496))
  - Polish ([#498](https://github.com/GyulyVGC/sniffnet/pull/498))
  - Romanian ([#499](https://github.com/GyulyVGC/sniffnet/pull/499))
  - Japanese ([#504](https://github.com/GyulyVGC/sniffnet/pull/504))
  - Uzbek ([#510](https://github.com/GyulyVGC/sniffnet/pull/510))
  - Swedish ([#522](https://github.com/GyulyVGC/sniffnet/pull/522))
- Reduced `String` allocations in translation code ([#524](https://github.com/GyulyVGC/sniffnet/pull/524))
- Fixed impossibility to exit thumbnail mode in some Linux distributions (fixes [#505](https://github.com/GyulyVGC/sniffnet/pull/505))

## [1.3.0] - 2024-04-08
- Introduced thumbnail mode, enabling users to keep an eye on Sniffnet while doing other tasks ([#484](https://github.com/GyulyVGC/sniffnet/pull/484))
- Added support for ICMP connections and messages ([#417](https://github.com/GyulyVGC/sniffnet/pull/417) — fixes [#288](https://github.com/GyulyVGC/sniffnet/issues/288))
- Added capability to identify 6000+ upper layer services, protocols, trojans, and worms ([#450](https://github.com/GyulyVGC/sniffnet/pull/450) — fixes [#374](https://github.com/GyulyVGC/sniffnet/issues/374))
- Added feature to optionally export the analysis as a PCAP file with a custom path ([#473](https://github.com/GyulyVGC/sniffnet/pull/473) — fixes [#162](https://github.com/GyulyVGC/sniffnet/issues/162) and [#291](https://github.com/GyulyVGC/sniffnet/issues/291))
- Introduced new filtering capabilities to allow users specify custom values of ports and IP addresses ([#414](https://github.com/GyulyVGC/sniffnet/pull/414))
- The size of text and widgets can now be customised by setting a proper zoom value (fixes [#202](https://github.com/GyulyVGC/sniffnet/issues/202) and [#344](https://github.com/GyulyVGC/sniffnet/issues/344))
- Added possibility to totally customize the app's theme via styles defined in TOML files ([#286](https://github.com/GyulyVGC/sniffnet/pull/286) and [#419](https://github.com/GyulyVGC/sniffnet/pull/419))
- Upgraded inspect page table: multiple new search filters, additional sorting options, and always keep a correct fields alignment ([#442](https://github.com/GyulyVGC/sniffnet/pull/442) — fixes [#63](https://github.com/GyulyVGC/sniffnet/issues/63))
- Added support for more link types in addition to Ethernet: raw IP packets and null/loopback packets are now correctly parsed ([#421](https://github.com/GyulyVGC/sniffnet/pull/421))
- Support changing sort strategy for network hosts and services in overview page, showing most recent items by default ([#452](https://github.com/GyulyVGC/sniffnet/pull/452))
- IP addresses can now be copied to clipboard from the popup related to a given entry of the connections table, and a new search parameter has been introduced in Inspect page to allow users filter their connections based on IP address values ([#409](https://github.com/GyulyVGC/sniffnet/pull/409))
- Traffic chart is now smoother and overall better-looking thanks to the new spline-based interpolation ([#461](https://github.com/GyulyVGC/sniffnet/pull/461))
- Added Japanese translation 🇯🇵 ([#343](https://github.com/GyulyVGC/sniffnet/pull/343))
- Added Uzbek translation 🇺🇿 ([#385](https://github.com/GyulyVGC/sniffnet/pull/385))
- Window size and position are now remembered, so that Sniffnet can reopen with the same window properties
- Users can now provide custom paths for MMDB files to allow using the commercial versions of the country and ASN databases (fixes [#243](https://github.com/GyulyVGC/sniffnet/issues/243))
- Added new command line option `--restore-default` to restore the default configurations of the app (settings, window properties, and device selected at startup)
- The app's configurations are now stored only on application close, instead of needlessly store them each time the settings popup is closed ([#420](https://github.com/GyulyVGC/sniffnet/pull/420))
- The textual output report is not generated anymore
- Settings "Language" tab has been removed. Language selection and other options are now included in a new settings tab "General" ([#365](https://github.com/GyulyVGC/sniffnet/pull/365))
- Updated Portuguese translation to v1.2 ([#398](https://github.com/GyulyVGC/sniffnet/pull/398))
- Cleaned code implementing the concept of first class theming ([#339](https://github.com/GyulyVGC/sniffnet/pull/339))
- Migrate to Iced 0.12 ([#470](https://github.com/GyulyVGC/sniffnet/pull/470))
- Added documentation about Sniffnet installation on Nix and Tiny Core Linux (respectively [#394](https://github.com/GyulyVGC/sniffnet/pull/394) and [#341](https://github.com/GyulyVGC/sniffnet/pull/341))
- General aesthetic improvements
- Fixed bug about not delivered favorite notifications in presence of old outgoing connections
- Fixed bug causing the application's icon not to be visible in some Linux environments
- Fixed a build failure on `powerpc64` ([#356](https://github.com/GyulyVGC/sniffnet/pull/356) — fixes [#353](https://github.com/GyulyVGC/sniffnet/issues/353))
- Fixed a typo in Russian translation ([#389](https://github.com/GyulyVGC/sniffnet/pull/389))
- Fixed icon inconsistency in case of directed broadcast traffic
- Made byte strings consistent across the app, and added support for Terabytes and Petabytes representations
- Fixed hosts and services data bar lengths inconsistencies in overview page
- Minor improvements to Spanish translation ([#454](https://github.com/GyulyVGC/sniffnet/pull/454))

## [1.2.2] - 2023-08-08
- Added option to set different shades of color gradients for each of the available themes
- Added new application themes: _Dracula_, _Gruvbox_, _Nord_, and _Solarized_ ([#330](https://github.com/GyulyVGC/sniffnet/pull/330))
- Other aesthetic improvements (see [#119](https://github.com/GyulyVGC/sniffnet/issues/119) for more info):
    - redesigned page tabs
    - highlighted headings with different colors
    - simplified scrollables style
    - improvements to Deep Sea and Mon Amour color palettes
- Added Finnish translation 🇫🇮 ([#310](https://github.com/GyulyVGC/sniffnet/pull/310))
- Added support for `--help` and `--version` command line arguments ([#272](https://github.com/GyulyVGC/sniffnet/pull/272))
- Migrated to [Iced 0.10](https://github.com/iced-rs/iced/releases/tag/0.10.0), that is now able to select the graphical renderer at runtime: a fallback one (`tiny-skia`) will be used in case the default one (`wgpu`) crashes ([#324](https://github.com/GyulyVGC/sniffnet/pull/324))
- Added app `id` in order to correctly show the icon and app name on Linux Wayland (fixes [#292](https://github.com/GyulyVGC/sniffnet/issues/292))
- Restructured issue templates to let users open issues in a more efficient and effective way ([#285](https://github.com/GyulyVGC/sniffnet/pull/285))
- Updated French translation to v1.2 ([#279](https://github.com/GyulyVGC/sniffnet/pull/279))
- Color palettes in settings page are now built as `Rule` widgets, without involving the use of external SVGs anymore
- Fixed `alt`+`tab` shortcut issue ([#298](https://github.com/GyulyVGC/sniffnet/pull/298) — fixes [#262](https://github.com/GyulyVGC/sniffnet/issues/262))
- Fixed problem that didn't allow opening links and the report file on operating systems different from Windows, macOS, and Linux
- Use scrollable to make active filters visible when the selected adapter name is long (overview page)
- Ensure no colored pixel is shown if the respective packets or bytes number is zero
- Minor fix to Chinese translation ([#271](https://github.com/GyulyVGC/sniffnet/pull/271))
- Where is Sniffnet heading next? See the new [roadmap of the project](https://github.com/GyulyVGC/sniffnet/blob/main/ROADMAP.md).

## [1.2.1] - 2023-06-08
- Considerably refined the app packaging strategy (see [#246](https://github.com/GyulyVGC/sniffnet/pull/246) for more details), fixing various related issues ([#199](https://github.com/GyulyVGC/sniffnet/issues/199), [#220](https://github.com/GyulyVGC/sniffnet/issues/220), [#223](https://github.com/GyulyVGC/sniffnet/issues/223), [#224](https://github.com/GyulyVGC/sniffnet/issues/224), [#225](https://github.com/GyulyVGC/sniffnet/issues/225), [#242](https://github.com/GyulyVGC/sniffnet/issues/242))
- Added button to clear all the current search filters quickly in inspect page
- Added Swedish translation 🇸🇪 ([#213](https://github.com/GyulyVGC/sniffnet/pull/213))
- Updated most of the existing translations to v1.2: 
  - German ([#191](https://github.com/GyulyVGC/sniffnet/pull/191))
  - Spanish ([#203](https://github.com/GyulyVGC/sniffnet/pull/203))
  - Persian ([#193](https://github.com/GyulyVGC/sniffnet/pull/193))
  - Korean ([#205](https://github.com/GyulyVGC/sniffnet/pull/205))
  - Polish ([#244](https://github.com/GyulyVGC/sniffnet/pull/244))
  - Romanian ([#241](https://github.com/GyulyVGC/sniffnet/pull/241))
  - Russian ([#187](https://github.com/GyulyVGC/sniffnet/pull/187))
  - Turkish ([#192](https://github.com/GyulyVGC/sniffnet/pull/192))
  - Ukrainian ([#216](https://github.com/GyulyVGC/sniffnet/pull/216))
  - Chinese ([#214](https://github.com/GyulyVGC/sniffnet/pull/214))
- Renamed "Administrative entity" to "Autonomous System name" to avoid confusion
- Improved filter columns relative width to avoid the "Application protocol" label being cut when displayed in Swedish
- Footer URLs have been updated to include links to Sniffnet's official website and GitHub Sponsor page
- Updated docs including installation instruction for Aarch Linux ([#185](https://github.com/GyulyVGC/sniffnet/pull/185))
- Minor improvements to packets and bytes number format
- Minor improvements to:
  - code readability ([#248](https://github.com/GyulyVGC/sniffnet/pull/248))
  - docs ([#235](https://github.com/GyulyVGC/sniffnet/pull/235))
- Solved a minor problem that caused flags to be slightly misaligned in inspect page table

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
