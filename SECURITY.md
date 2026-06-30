# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest  | Yes       |
| < latest | No (upgrade to latest) |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, use [GitHub Security Advisories](https://github.com/srothgan/tui-textarea/security/advisories/new)
to report vulnerabilities privately.

Please include:

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Suggested fix, if any

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix and disclosure**: Coordinated with reporter, typically within 30 days

## Scope

This policy covers the `tui-textarea-2` crate and vulnerabilities in its integration
with supported terminal backends and optional features. Vulnerabilities in upstream
libraries such as Ratatui, Crossterm, Termion, or Termwiz should be reported to
their respective maintainers unless the issue is caused by this crate's usage or
integration of those libraries.

## Security Measures

- Dependency updates are managed via Dependabot
- Dependabot security alerts and security updates are enabled
- All PRs require CI checks for test, clippy, fmt, and MSRV compatibility
