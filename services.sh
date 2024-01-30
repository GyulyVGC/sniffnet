#!/usr/bin/env zsh

# Script used to generate the file ./src/networking/services.rs
# The source for port-to-service mappings is provided by nmap
# Available at: https://raw.githubusercontent.com/nmap/nmap/master/nmap-services

OUT=./src/networking/services.rs

{
  printf '//! This file is generated automatically via ./services.sh\n\n'
  printf 'use crate::networking::types::protocol::Protocol;\n\n'
  printf '#[allow(clippy::too_many_lines, clippy::match_same_arms)]\n'
  printf 'pub fn get_service(port_info: (u16, Protocol)) -> String {\n'
  printf '\tmatch port_info {\n'
} > $OUT

curl https://raw.githubusercontent.com/nmap/nmap/master/nmap-services \
 | grep -E '/tcp|/udp' \
 | grep -E -v '^unknown|^#' \
 | cut -d$'\t' -f 1,2 \
 | tr '/' '\t' \
 | awk -F $'\t' '{printf("\t\t(%s, Protocol::%s) => \"%s\",\n", $2, toupper($3), $1)}' \
 >> $OUT

{
  printf '\t\t(_, _) => "?",\n'
  printf '\t}.to_string()\n'
  printf '}\n'
} >> $OUT
