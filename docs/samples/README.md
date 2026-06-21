# DNS sample capture

`dns_sample.pcap` is a tiny, self-contained capture used to demonstrate and
validate the DNS analyzer offline (no live traffic needed).

It contains two Ethernet/IPv4/UDP frames on port 53:

1. **Query** — `google.com`, type A (transaction id `0x1234`).
2. **Response** — `google.com` A → `8.8.8.8`, TTL 300, using a DNS
   name-compression pointer; sent ~18 ms after the query.

## How to use

1. Open Sniffnet and import this file (capture from file).
2. Open the **DNS** tab: you should see one `Q` and one `R` row for
   `google.com`, type `A`, RCODE `NOERROR`, the answer `8.8.8.8`, and a
   resolution latency of ~18 ms.
3. Optionally open the same file in Wireshark and compare the dissected
   fields (id, flags, QNAME, QTYPE, RCODE, RDATA) — they must match.

The same byte vectors are exercised by the unit tests in
`src/networking/dns/parser.rs`.
