# Incident Response Plan

This document describes how Sniffnet handles security incidents.

For how to report a vulnerability, see [`SECURITY.md`](https://github.com/GyulyVGC/sniffnet/blob/main/SECURITY.md).<br>
For the project's threat model, see [`THREAT_MODEL.md`](https://github.com/GyulyVGC/sniffnet/blob/main/THREAT_MODEL.md).

## Scope

An incident is anything that compromises, or could compromise, the integrity
or safety of Sniffnet or its users:

- a vulnerability in Sniffnet or one of its dependencies
- a tampered release artifact or compromised signing key
- unauthorized commits, tags, or releases
- a compromised maintainer account or the [sniffnet.net](https://sniffnet.net) website

## Roles

- **Incident Lead**: the project maintainer
  ([@GyulyVGC](https://github.com/GyulyVGC)). Owns the response and all
  external communication.
- **Reporter**: credited in the final advisory unless they prefer otherwise.
- **Trusted contributors**: may be invited into the private advisory to help
  fix or review.

## Process

1. **Triage.** Open a private
   [GitHub Security Advisory](https://github.com/GyulyVGC/sniffnet/security/advisories)
   to track the issue. Confirm it reproduces and assess impact.
2. **Contain.** If needed, withdraw affected releases, revoke exposed secrets
   or keys, or revert unauthorized changes.
3. **Fix.** Develop and review the patch privately. Add a regression test
   where possible. Rotate anything that might have been exposed.
4. **Release.** Ship a patched version through the usual channels.
5. **Disclose.** Publish the advisory (request a CVE if applicable) and note
   the fix in [`CHANGELOG.md`](https://github.com/GyulyVGC/sniffnet/blob/main/CHANGELOG.md).
6. **Review.** Write a short post-mortem and file follow-up issues for any
   gaps it exposes.

Nothing about an unfixed issue is discussed in public issues, PRs, or commit
messages until disclosure.
