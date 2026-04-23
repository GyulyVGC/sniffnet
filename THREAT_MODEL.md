# Threat Model

This is Sniffnet's first threat model.<br>
The model is intentionally scoped to a subset of the application — future iterations will expand coverage.

We'll use STRIDE, a widely adopted framework to help identify common types of threats:

- **Spoofing**: Impersonating someone or something else (e.g., faking identity).
- **Tampering**: Modifying data or code (e.g., altering a file, changing network packets).
- **Repudiation**: Denying an action that occurred (e.g., a user denies making a transaction).
- **Information Disclosure**: Revealing sensitive data to unauthorized individuals (e.g., leaking PII, credentials).
- **Denial of Service**: Preventing legitimate users from accessing a service or resource (e.g., crashing a server, exhausting resources).
- **Elevation of Privilege**: Gaining unauthorized access to higher-level permissions (e.g., a regular user becoming an administrator).

For how to report a vulnerability, see [`SECURITY.md`](https://github.com/GyulyVGC/sniffnet/blob/main/SECURITY.md).<br>
For how incidents are handled, see [`INCIDENT_RESPONSE.md`](https://github.com/GyulyVGC/sniffnet/blob/main/INCIDENT_RESPONSE.md).

## 1. Project overview and scope of this document

- **Project name:** Sniffnet
- **Brief description:** Cross-platform desktop application to monitor Internet traffic.
- **Key features in scope of this model:**
  - Packet capture and parsing from a local network adapter or imported PCAP file
  - User-provided files (custom MaxMind MMDB databases, IP blacklists, custom
    theme palettes)
  - Persisted and reloaded app configurations (settings, favorites, notification preferences, etc.)
  - Remote notifications sent to a user-configured HTTP endpoint

## 2. Assets to protect

| Asset                        | Why it matters                                                                                                                                                    |
|:-----------------------------|:------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Host system integrity**    | Sniffnet processes untrusted inputs (network traffic, user files). A vulnerability could lead to compromise of the user's system.                                 |
| **User's data privacy**      | Sensitive data (including the captured traffic metadata and custom app configurations) could be exposed to a third party via a vulnerability or misconfiguration. |
| **Application availability** | A crash during a monitoring session could cause loss of visibility into the user's network activity at a critical moment.                                         |
| **Project reputation**       | A vulnerability that leads to compromise of user systems or data would damage trust in the project and its maintainers.                                           |

## 3. Threats (STRIDE)

| # | Component / Flow           | Spoofing | Tampering  | Repudiation | Information disclosure | Denial of Service | Elevation of Privilege |
|:-:|:---------------------------|:--------:|:----------:|:-----------:|:----------------------:|:-----------------:|:----------------------:|
| 1 | Packet capture and parsing |    —     |     T1     |      —      |           I1           |        D1         |           E1           |
| 2 | Custom MMDB file           |    —     |     T2     |      —      |           —            |         —         |           —            |
| 3 | IP blacklist file          |    —     |     T3     |      —      |           —            |        D3         |           —            |
| 4 | Custom palette file        |    —     |     T4     |      —      |           —            |         —         |           —            |
| 5 | App configurations file    |    —     |     T5     |      —      |           I5           |         —         |           —            |
| 6 | Remote notifications POST  |    S6    |     —      |      —      |           I6           |         —         |           —            |

### Threat catalog

- **T1 — Malformed packet triggers parser bug.** A crafted frame on the wire
  reaches `LaxPacketHeaders::from_ethernet` / `from_ip` / `from_ether_type`.
  A logic bug in `etherparse` (or in Sniffnet's post-parse handling) could
  produce memory-safety issues, incorrect attribution, or panics.
- **I1 — PCAP export discloses captured traffic.** When the user enables
  `Savefile`, captured packets — headers, hostnames, DNS queries, and
  payloads — are written to disk unencrypted and can be read by any
  local process.
- **D1 — Packet flood DoS.** High packet rates or many small packets force
  the parser thread to fall behind; GUI freezes or memory grows unboundedly.
- **E1 — Privilege escalation via parser RCE.** If T1 becomes exploitable,
  the attacker gets code execution inside a process holding `CAP_NET_RAW`
  (Linux) or Npcap driver access (Windows). There is no privilege drop
  after capture handle creation.
- **T2 — Malicious MMDB.** A crafted `.mmdb` targets the `maxminddb` reader
  (tree traversal, decoder). `open_readfile` memory-maps the file, so
  parser bugs touch privileged memory.
- **T3 — Blacklist tampering.** Another process running as the user can
  edit the file loaded by `IpBlacklist::from_file` to remove entries for
  known-bad IPs (suppressing alerts) or add benign IPs (driving false
  positives). There is no signature, checksum, or integrity check on the
  blacklist contents.
- **D3 — Huge blacklist file.** `IpBlacklist::from_file` loads the whole
  file into memory via `tokio::fs::read_to_string` — no streaming, no size
  cap. A multi-GB file would exhaust memory.
- **T4 — Palette tampering.** `Palette::from_file` accepts any well-formed
  hex color without semantic validation. A tampered TOML can set alert
  colors to be indistinguishable from normal-traffic colors, or render
  the UI illegibly, causing the user to miss or misattribute
  security-relevant GUI state.
- **T5 — Config tampering.** Another process running as the user can edit
  `conf.toml` to insert a malicious MMDB path, blacklist path, palette
  path, or remote webhook URL. On next launch Sniffnet trusts these paths.
- **I5 — Config discloses watch list.** Favorites, custom
  file paths, and the webhook URL can be read by any local process.
- **S6 — Webhook impersonation.** A tampered `conf.toml` redirects
  notifications to an attacker's server while the user still believes
  they're sending to their own bot / SIEM. The JSON body
  (`LoggedNotification::to_json`) contains host info, service, favorite
  metadata, and byte counts — i.e., the user's own network activity.
- **I6 — Webhook body discloses monitored assets.** The JSON emitted by
  `LoggedNotification::to_json` includes favorited host country / ASN /
  domain and, for blacklist hits, the malicious IP, its reverse-resolved
  domain, and byte counts. A compromised webhook endpoint — or an
  on-path observer if the URL is plaintext — learns which assets the
  user watches and which security events have fired.

## 4. Mitigations

|  #  | Threat       | Mitigation                                                                                                                  | Status                                        |
|:---:|:-------------|:----------------------------------------------------------------------------------------------------------------------------|:----------------------------------------------|
| M1  | T1 / E1 / T2 | Keep `etherparse`, `pcap`, and `maxminddb` on the latest patched versions.                                                  | Active — Dependabot monitors Cargo manifests. |
| M2  | T1 / E1      | Add fuzz targets for the packet ingestion entry points (`from_ethernet`, `from_null`, `from_linux_sll`) using `cargo-fuzz`. | Not implemented.                              |
| M3  | D1           | Bound the pcap channel / parsing queue and drop oldest rather than growing memory unboundedly.                              | Needs review of `pcap_tx` backpressure.       |
| M4  | D3           | Cap the blacklist file size (e.g. 10 MB) before `read_to_string`.                                                           | Not implemented.                              |
| M5  | I6           | Reject non-`https://` webhook URLs in the settings UI.                                                                      | Not implemented.                              |
