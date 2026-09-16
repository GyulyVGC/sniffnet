# Service taxonomy research

Research date: September 16, 2026. Repository input: `services.txt` at `592a62a5ad2fb2e9f8470a6174ab9f18dc86997a`.

## Recommendation

Use **20 purpose categories plus one fallback, Other**, with one category per service name. The category describes the named service's expected function; a port lookup does not establish the actual protocol or application using that port.

The full input contains **6,465 distinct service names and 12,093 TCP/UDP mappings**. Every name has been assessed against the available registry descriptions and is present in `services.csv`. The resulting research mapping assigns **3,270 names to a specific category** and **3,195 to Other**.

**This is a reviewed research mapping, not a claim that all 6,465 service identities have been independently verified.** Some assignments interpret well-known protocol/product names; external protocol or vendor documentation was consulted for selected ambiguous/important cases. Other intentionally includes unclear or poorly documented names. Completeness of rows must not be confused with complete semantic verification. The evidence file preserves those limits for further review.

## Categories

Counts are distinct service names in this mapping, **not traffic volume, prevalence, importance, or a complete census of each domain**. Entries left in Other could change these counts after further research.

| ID            | Category                       | Names | Scope                                                                                                                                                                  |
|---------------|--------------------------------|------:|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `web`         | Web                            |    68 | Web content, generic web APIs, and web transfer protocols. Examples: http, https, coap.                                                                                |
| `email`       | Email                          |    53 | Email delivery, access, notification, and mail-specific routing. Examples: smtp, imap, pop3.                                                                           |
| `chat`        | Chat & messaging               |    90 | Human messaging, chat, discussion/news, SMS and paging. Examples: irc, xmpp-client, matrix-fed.                                                                        |
| `media`       | Voice & media                  |   229 | Audio/video delivery, calls, conferencing, and audiovisual sessions. Examples: sip, rtsp, rtmp.                                                                        |
| `files`       | File transfer & sharing        |   131 | File/object transfer, synchronization, sharing, and network filesystems. Examples: ftp, rsync, nfs, microsoft-ds.                                                      |
| `storage`     | Storage & backup               |   125 | Storage infrastructure, block/tape access, backup, restore, and protected replication. Examples: iscsi, ndmp, bacula-sd.                                               |
| `database`    | Databases & caches             |   134 | Database/query engines, data caches, database access and replication. Examples: mysql, postgresql, redis.                                                              |
| `remote`      | Remote access                  |   125 | Interactive computer/desktop/application access and remote execution. Examples: ssh, telnet, vnc, ms-wbt-server.                                                       |
| `management`  | Monitoring & administration    |   853 | IT monitoring, logs, diagnostics, configuration, deployment, availability, and resource management. Examples: snmp, syslog, netconf-ssh, zabbix-agent.                 |
| `discovery`   | Naming & discovery             |   158 | Name resolution and locating network devices/services. Examples: domain, mdns, llmnr, svrloc.                                                                          |
| `network`     | Network infrastructure         |   268 | Address assignment, network boot, time synchronization, routing, switching, mobility, and telecom/network control. Examples: dhcps, ntp, bgp, vxlan.                   |
| `vpn`         | VPN, tunnels & proxies         |    81 | General connection tunneling, network relays, VPN negotiation, and proxies. Examples: openvpn, isakmp, socks, http-proxy.                                              |
| `identity`    | Identity & directory           |   152 | Authentication, authorization, directories, identity lookup, and credential/key/certificate services. Examples: kerberos, ldap, radius, kmip.                          |
| `printing`    | Printing & scanning            |    49 | Printing, document scanning, fax, and their dedicated peripheral protocols. Examples: ipp, printer, jetdirect, sane-port.                                              |
| `gaming`      | Gaming                         |    65 | Game sessions, multiplayer transports, game lobbies, and game-specific communication. Examples: minecraft, quake3, xbox.                                               |
| `industrial`  | Industrial & device automation |   137 | Physical process/equipment control, building/home automation, industrial telemetry and metering. Examples: mbap, bacnet, opcua-tcp, zigbee-ip.                         |
| `development` | Software development           |    48 | Source control, compilation/build/test tools, debugging, and development support. Examples: git, svn, distcc, gdbremote.                                               |
| `middleware`  | Application middleware         |   245 | General application messaging, RPC, distributed objects, transaction coordination and shared application infrastructure. Examples: amqp, mqtt, msrpc, rtps-dd-ut.      |
| `security`    | Security tools                 |    86 | Threat/vulnerability detection, prevention, integrity checking, security policy enforcement and incident exchange. Examples: nessus, spamassassin, prelude, idxp.      |
| `licensing`   | Software licensing             |   173 | Software license distribution, entitlement verification, activation, and floating-license services. Examples: flexlm, CodeMeter, WibuKey, keysrvr.                     |
| `other`       | Other                          | 3,195 | Single fallback for specialized applications outside this taxonomy, ambiguous identities, or insufficiently documented purposes. Examples: dicom, bitcoin, NFS-or-IIS. |

## Why these categories

- Keep familiar functions separate: Web, Email, Chat, Voice/media, Files, Remote access, Printing, and Gaming explain common activities without knowing the product name.
- Keep Naming/discovery separate from Network infrastructure: finding a service is a different purpose from addressing, routing, or keeping time.
- Keep Databases, Identity, Middleware, and Monitoring separate: these are distinct functions across products and vendors, including on development and enterprise machines.
- Combine Storage and Backup: both concern storage infrastructure and data protection; file sharing remains separate because accessing files is a distinct user-facing purpose.
- Add Software licensing: the sources explicitly identify a substantial, coherent family of license services. Do not infer licensing solely from an unexplained `-lm` suffix.
- Keep Industrial/device automation and Security tools because their purposes and consequences are distinctive, even without evidence that they dominate ordinary desktop traffic.
- Do not add Business applications, Scientific/medical, Cryptocurrency, or Distributed computing merely to reduce the fallback count. Specialized application workflows go to Other; documented cluster/job administration fits Monitoring, and generic distributed application communication fits Middleware. Insufficiently documented cluster labels remain Other.

This selection is a judgment about semantic usefulness and Sniffnet's broad audience. No current application-popularity measurement is available from these files. Nmap's port-open frequency was used only to prioritize a review queue, never to infer application identity or claim current service popularity.

## Assignment boundaries

1. Classify the named service's function, not its vendor, transport, encryption, or a guessed use case. HTTPS remains Web; it does not reveal whether the payload is video, chat, or backup.
2. Dedicated functionality takes precedence over generic transport: `netconf-ssh` is Monitoring, `rtps-dd-ut` is Middleware, and `hp-pdl-datastr` is Printing. MQTT remains Middleware despite common IoT usage; CoAP remains Web.
3. Where the endpoint explicitly provides administration or monitoring, classify that role: `mysql-im` is Monitoring, while `mysql` is Databases. Dedicated security-product operations stay Security tools. Authentication, credential issuance and software licensing have their own categories.
4. Do not apply an unrelated IANA occupant's purpose to an older Nmap service name at the same port. Preserve ambiguity when the name itself combines alternatives: `NFS-or-IIS` stays Other.
5. A family can legitimately span categories: `sms-remctrl` is Remote access, `sms-chat` is Chat, and `sms-xfer` is Files. Here SMS means Microsoft's Systems Management Server, not text messaging.
6. The same name can have different registered purposes at different ports. `npp` covers both printing and paging, so this name-level mapping uses Other. An endpoint-level taxonomy could split it in a future design.
7. Security tools means a documented security function. TLS does not put a service there. A known remote-control malware service may still describe Remote access; its category is never a verdict about observed traffic.

## Evidence and verification

- The Nmap source was matched to the repository's exact service name and port/protocol combinations. Its comments often include alternative historical port occupants; these were treated as evidence requiring interpretation, not as interchangeable aliases.
- IANA entries were joined by service name (case-insensitive), retaining descriptions and references. A same-number/different-name match was not substituted. IANA had same-name descriptions for 5,921 names; either source had a description for 6,376.
- Cisco’s protocol reference was also checked for matching names. Its category labels are preserved separately in the evidence, including disagreements. Name matches are corroborating leads, not proof of identity: apparent collisions such as `isis`, `send`, and `nmap` were not used to force a category.
- Narrow text rules produced initial review suggestions. All names/descriptions and category conflicts were reviewed; corrections and conservative fallbacks were applied. The output is an explicit mapping, not a keyword classifier intended for application use.
- The IANA registry does not supply this taxonomy. Every category assignment is an interpretation. Additional protocol/vendor documentation and Cisco reference pages are linked per service where consulted; source descriptions and references remain visible in `evidence.csv`.
- Verified mechanically: no missing or extra names; no duplicate names; one permitted category per name; all 12,093 input endpoints represented in the evidence; category totals sum to 6,465. No application code, generated service list, or UI changed.

## Files

- `services.csv`: complete proposed name-to-category mapping.
- `evidence.csv`: complete service-level audit, including endpoints, source descriptions, rationale and additional documentation links. Cisco category labels are reference data and do not necessarily agree with the proposed category.
- `sources.json`: source URLs and SHA-256 hashes, repository revision, coverage counts and consulted additional sources.

Start any subsequent refinement with the evidence for Other and the terse name-based assignments. Do not reduce Other by guessing product purpose or broadening Middleware/Monitoring into catch-all categories.
