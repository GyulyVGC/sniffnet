# sniffnet-packet-engine

Link-layer and packet header parsing shared by [Sniffnet](https://github.com/GyulyVGC/sniffnet)
and [Sniffnet Agent](https://github.com/GyulyVGC/sniffnet-agent).

`parse` turns a raw `pcap` packet into a `ParsedPacket`: addresses, ports, protocol,
MACs, and byte count. Flow aggregation, traffic direction, service lookup and IPFIX
export stay with the caller.

```rust
use sniffnet_packet_engine::{LinkType, parse};

let link_type = LinkType::from_pcap(cap.get_datalink());
while let Ok(packet) = cap.next_packet() {
    if let Some(parsed) = parse(packet.data, link_type) {
        println!("{} {} -> {}", parsed.protocol, parsed.addrs.src(), parsed.addrs.dst());
    }
}
```

## Supported link types

`ETHERNET`, `NULL`, `LOOP`, `IPV4`, `IPV6`, `RAW`, `LINUX_SLL`, `LINUX_SLL2`.

Anything else is reported by `LinkType::is_supported` as unsupported and parsed as
Ethernet; callers that would rather skip such captures check first.

## License

MIT OR Apache-2.0
