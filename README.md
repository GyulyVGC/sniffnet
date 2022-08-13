# Network analyzer (packet sniffer)

The executable file path is packet_sniffer/target/debug/packet_sniffer


## Command line options

 - [-a, --adapter]
Name of the network adapter to be inspected, if omitted the default adapter is chosen.
This option must be followed by a textual value.
 
 - [-d, --device-list]
 Prints list of the available devices. 
 Immediately terminates the program.
 This option does not need to be followed by a value.
 
 - [-h, --highest-port]
Sets the maximum port value to be considered, if omitted there is not ports higher bound.
This option must be followed by an integer value between 0 and 65535. 
 [default: 65535]
 
 -  [-i, --interval]
 Sets the interval of time between report updates (value in seconds).
 This option must be followed by a positive integer value.
 [default: 5]
 
 - [-l, --lowest-port]
Sets the minimum port value to be considered, if omitted there is not ports lower bound.
This option must be followed by an integer value between 0 and 65535. 
 [default: 0]
 
 - [-m, --minimum-packets]
Sets the minimum value of transited packets for an address:port to be printed in the report.
This option must be followed by a positive integer value.
 [default: 0]
 
 - [-o, --output-file]
Name of output file to contain the textual report, if omitted a default file is chosen.
This option must be followed by a textual value.
[default: report.txt]


## Error conditions

...

