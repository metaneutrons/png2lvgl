# Security Policy

## Reporting a vulnerability

Report vulnerabilities through GitHub's **Private Vulnerability Reporting**:

1. Open the [security advisories][advisories] page
2. Choose **Report a vulnerability**

[advisories]: https://github.com/metaneutrons/png2lvgl/security/advisories

This keeps the report private until a fix is available. Please do not open a
public issue for a security problem, and please do not send reports by email so
that the whole exchange stays attached to the advisory.

Include the version (`png2lvgl --version`), the input that triggers the problem
and what you expected to happen instead. A minimal PNG that reproduces the
behaviour is the most useful thing you can attach.

## What to expect

| Step | Timeframe |
|---|---|
| Acknowledgement of your report | within 5 working days |
| Initial assessment with severity | within 10 working days |
| Fix released, or a dated plan | agreed with you in the advisory |

Fixes ship as a normal patch release through every channel listed in the
README. The advisory is published once the release is available, crediting you
unless you ask otherwise.

## Supported versions

Only the latest release receives fixes. The project is pre-1.0, so please
upgrade to the current version before reporting.

| Version | Supported |
|---|---|
| latest release | yes |
| anything older | no |

## Scope

png2lvgl is a command line tool that reads PNG files and writes C source. The
interesting attack surface is therefore untrusted image input: parser crashes,
unbounded allocation, integer overflow in the size calculations, path handling
of the output file, and generated C that does not compile or does not match the
input.

Vulnerabilities in dependencies are handled here as well. If the issue is
entirely inside a dependency, a report to that project reaches more users, but
do tell us either way so the version can be raised.
