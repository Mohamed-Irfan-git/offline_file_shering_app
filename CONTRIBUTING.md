# Contributing to LAN Share

Thank you for helping improve LAN Share. This project is meant to be simple, reliable, and friendly on phones and desktops alike.

## Before you start

1. Read the [README](README.md) — especially **How it works** and **Security notice**.
2. Run the app locally: `cargo run` and test with a phone on the same Wi‑Fi.
3. Check existing issues and PRs to avoid duplicate work.

## Development setup

```bash
git clone <repo-url>
cd lan-share
cargo run
```

- UI lives in `static/app.js` and `static/style.css`.
- HTML shell: `src/templates/html.rs`.
- API routes: `src/routes/`.
- Default port: **5000** (`src/utils/network.rs`).

Use a release build when testing large uploads:

```bash
cargo build --release
./target/release/lan-share
```

## What to work on

We welcome contributions in these areas (pick one or propose your own):

### User experience

- Dark/light theme toggle  
- Better empty states and onboarding  
- Upload queue: pause, cancel, retry  
- Haptic / visual feedback on mobile  
- Accessibility (keyboard nav, ARIA, contrast)

### Features

- **Room codes or PIN** — optional session password  
- **mDNS** — `http://lan-share.local:5000` instead of typing IP  
- **Download all as ZIP**  
- **Folder upload** (where browsers support it)  
- **Transfer history** per session  
- **Auto-delete** files after N hours  
- **Multiple share rooms** on different ports

### Packaging & distribution

- GitHub Actions: build artifacts for Windows / macOS / Linux  
- `.deb` / `.rpm` / Homebrew formula  
- Windows installer (WiX / NSIS)  
- macOS `.app` bundle  
- Single-binary embed static assets (optional)

### Quality

- Integration tests for upload/download API  
- Unit tests for `network.rs`, `file_name.rs`  
- Fuzzing or path-traversal security tests  
- Document firewall steps per OS

### Documentation

- Screenshots / GIFs in README  
- Translations (README + UI strings)  
- Video: “phone to laptop in 60 seconds”

## How to submit changes

1. **Fork** the repository.  
2. Create a branch: `feature/short-description` or `fix/issue-description`.  
3. Keep PRs **focused** — one feature or fix per PR when possible.  
4. Test on at least:
   - Host desktop browser  
   - One phone on LAN (real device beats emulator)  
5. Open a **Pull Request** with:
   - What changed and why  
   - How you tested it  
   - Screenshots for UI changes  

## Code guidelines

- **Rust:** match existing style; avoid large unrelated refactors.  
- **JS:** keep compatible with LAN HTTP (no `crypto.randomUUID` without fallback).  
- **CSS:** use existing CSS variables in `:root` when possible.  
- **Security:** validate paths; never serve files outside `uploads/`.  
- **Mobile:** test file picker with `<label for="fileInput">`, not only programmatic `.click()`.

## Reporting bugs

Include:

- OS (Windows 11, macOS 14, Ubuntu 24.04, Android 14, iOS 17, …)  
- Browser (Chrome, Safari, Firefox, …)  
- Host or guest? URL used (`localhost` vs `192.168.x.x`)  
- Steps to reproduce  
- What you expected vs what happened  

## Suggesting ideas

Open an issue with:

**Title:** `Idea: <short summary>`

**Body:**

- Problem you’re solving  
- Proposed solution  
- Alternatives considered  
- Willing to implement? (yes / need help)

We’ll discuss before large architectural changes.

## Code of conduct

Be respectful and constructive. LAN Share is a community tool — help others learn.

---

**Questions?** Open an issue labeled `question`. We’re glad you’re here.
