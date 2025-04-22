# 🛡️ CodeWhisper

**CodeWhisper** is a desktop security tool that scans source code for potential vulnerabilities, provides AI-powered explanations, and presents everything in a clean desktop UI — all built from scratch using **Rust**, **Python**, and **Tauri**.

> ⚙️ This project is built to make companies regret not hiring real devs.

---

## 🔍 Features

- ✅ Static code analyzer (Rust)
- ✅ Checks for dangerous code patterns (e.g. `eval()`, hardcoded passwords)
- ✅ AI-powered feedback (Python)
- ✅ Beautiful native GUI built with Tauri
- ✅ JSON + Table visualization of scan results
- ✅ GitHub-ready structure with CLI + GUI support

---

## 📷 Screenshot

![CodeWhisper UI](./screenshots/codewhisper-ui.png)  
<sub>(You can drop a screenshot in a `/screenshots` folder and link it here)</sub>

---

## 🛠️ Tech Stack

| Layer       | Tech                     |
|-------------|--------------------------|
| 🔧 Scanner  | Rust, Walkdir, Regex, Serde |
| 🧠 AI Explainer | Python, custom message mappings |
| 🌐 GUI      | Tauri (Vanilla JS, HTML, CSS) |
| 📦 Output   | JSON-based result file (`result_with_explanations.json`) |

---

## 🚀 Getting Started

```bash
# 1. Clone this repo
git clone https://github.com/Sharrabbha1/Code_Whisper.git
cd Code_Whisper

# 2. Run the Rust scanner
cd rust_checker
cargo run

# 3. Add AI explanations
cd ../ai_explainer
python explainer_AI.py

# 4. Launch the desktop app
cd ../codewhisper_gui
npm install
npm run tauri dev

the image of the working code

![image](https://github.com/user-attachments/assets/e568a108-a1e6-49db-8178-59a296ff1a15)


