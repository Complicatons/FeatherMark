## What changed

Describe the user-visible outcome and why the change belongs in FeatherMark's deliberately small scope.

## Verification

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml`
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- [ ] `node --check src/app.js`
- [ ] `npm.cmd run check:themes`
- [ ] `npm.cmd run build`
- [ ] I manually checked the affected behaviour.

## Evidence and limitations

Include screenshots for interface changes, note any fixture added, and state anything you could not verify.

## Security and dependencies

- [ ] The change does not weaken raw-HTML, link, image-path, file-access, or CSP protections.
- [ ] No dependency was added, or its size and security cost is justified above.
