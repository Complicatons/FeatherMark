# Security policy

## Supported versions

FeatherMark is early-stage software. Security fixes are applied to the latest released version and the current `main` branch.

| Version | Supported |
| --- | --- |
| 0.1.x | Yes |
| Older versions | No |

## Reporting a vulnerability

Please do not open a public issue for a suspected vulnerability.

Use GitHub's **Report a vulnerability** option in the repository's Security tab to send a private report. Include:

- The affected FeatherMark version and Windows version.
- A minimal Markdown file or exact reproduction steps.
- The impact you observed or believe is possible.
- Any suggested mitigation, if you have one.

Remove unrelated personal or confidential information before attaching a document. A maintainer will acknowledge the report when it is reviewed, investigate it, and coordinate disclosure after a fix is available. No fixed response deadline is promised while the project is maintained by a small team.

## Security boundaries

FeatherMark treats Markdown as untrusted, escapes raw HTML, blocks unsafe schemes and remote images, confines relative image access to the document directory, and applies a restrictive Content Security Policy. These controls are security boundaries; changes affecting them require focused tests and explicit review.
