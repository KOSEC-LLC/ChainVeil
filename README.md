<table style="border: none; margin: 0 auto; padding: 0; border-collapse: collapse;">
<tr>
<td align="center" style="vertical-align: middle; padding: 10px; border: none; width: 220px;">
<img src="assets/chainveil-logo.png" alt="ChainVeil Logo" width="180"/>
</td>
<td align="left" style="vertical-align: middle; padding: 10px 0 10px 30px; border: none;">
  <pre style="font-family: 'Courier New', monospace; font-size: 15px; color: #22d3ee; margin: 0; padding: 0; text-shadow: 0 0 10px #22d3ee, 0 0 20px rgba(34,211,238,0.5); line-height: 1.2; display: block;">
_________ .__           .__     ____   ____     .__.__   
\_   ___ \|  |__ _____  |__| ___\   \ /   /____ |__|  |  
/    \  \/|  |  \\__  \ |  |/    \   Y   // __ \|  |  |  
\     \___|   Y  \/ __ \|  |   |  \     /\  ___/|  |  |__
 \______  /___|  (____  /__|___|  /\___/  \___  >__|____/
        \/     \/     \/        \/            \/         
  </pre>
</td>
</tr>
</table>

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E)
[![License](https://img.shields.io/badge/License-AGPL%20V3-blue?style=for-the-badge)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/KOSEC-LLC/ChainVeil?style=for-the-badge\&logo=github)](https://github.com/KOSEC-LLC/ChainVeil/stargazers)

ChainVeil is a **Rust-powered C2 Framework** for generating custom JavaScript payloads that interact with Ethereum. ChainVeil is designed to demonstrate how threat actors leverage decentralized technology for its flexibility and robustness.

---

## ⚡ Features

* **Rust-powered payload creation** — ultra-lightweight, fast, and cross-platform.
* **Custom JS generation** — generate JavaScript that fetches data from Ethereum addresses.
* **Portable execution** — compile a single binary or run via script so no heavy dependencies.
* **Community templates** — contribute your own templates to expand functionality.

---

## 🛠 Usage

1. Clone the repository:

```bash
git clone https://github.com/KOSEC-LLC/ChainVeil.git
cd ChainVeil
```

2. Build the Rust binary:

```bash
cargo build --release
```

3. Run ChainVeil with your Ethereum address:

```bash
./target/release/ChainVeil \
  --content "0x000000000000000000000000000000000000dEaD" \
  --template templates/downloader.js \
  --output output/example.js \
  --payload payloads/payload.ps1
```

4. Open or run the generated `output/example.js` in your JS environment to fetch and log data.

---

## 📂 File Structure

```
ChainVeil/
├── src/
│   └── main.rs          # Rust script that powers JS generation
├── templates/
│   └── downloader.js    # JS template with placeholder
├── output/              # Generated JS files
├── payloads/
│   └── payload.ps1      # PowerShell payload placeholder
├── assets/
│   └── logo.png         # Logo for README
├── Cargo.toml           # Rust dependencies
└── README.md
```

---

## 🎨 Customization

* Swap templates in `templates/` for your own JS payloads.
* Swap payloads in `payloads/` for your own customized payloads.
* Inject multiple addresses by extending the Rust script.
* Integrate with your security tooling workflow or Ethereum research projects.

---

## ⚠️ Disclaimer

ChainVeil is intended for **educational, research, and internal security purposes only**. Do **not** deploy against external systems without permission. KOSEC assumes no liability for misuse.
