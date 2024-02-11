#!/bin/bash

# Script used to generate the file ./services.txt
# The source for port-to-service mappings is provided by nmap
# Available at: https://raw.githubusercontent.com/nmap/nmap/master/nmap-services

OUT=./services.txt

curl https://raw.githubusercontent.com/nmap/nmap/master/nmap-services \
  | grep -E '/tcp|/udp' \
  | grep -E -v '^#|^unknown\t' \
  | cut -d$'\t' -f 1,2 > $OUT
