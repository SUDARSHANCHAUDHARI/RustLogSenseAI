# LogSenseAI 🖥️🔍

**Paste device logs and get instant AI diagnostics — root cause, error clusters, timeline, customer reply, and engineering notes, all in one click.**

Paste or upload any digital signage device log file, select the device type, and get a complete structured analysis: severity rating, error and warning counts, clustered error patterns with occurrence counts, root cause explanation, incident timeline, a copy-ready customer reply, detailed engineering notes, and prioritized next steps.

---

## 🌟 Features

### 🖥️ Core Features
- ✅ **Log Input** — paste raw log output directly into the textarea
- ✅ **File Upload** — upload `.log` or `.txt` files directly
- ✅ **Demo Logs** — one-click load of a sample SSL / cache failure log
- ✅ **Device Type Selector** — BrightSign, Samsung SSSP, LG webOS, Android, Raspberry Pi, Chrome OS, Windows, Unknown
- ✅ **Line + Character Counter** — live count of log size
- ✅ **Severity Badge** — Critical / High / Medium / Low / Info
- ✅ **Error + Warning Counts** — numeric totals
- ✅ **Error Clusters** — grouped recurring patterns with occurrence count, first/last seen, example line
- ✅ **Root Cause Panel** — plain-English explanation of what caused the errors
- ✅ **Timeline** — chronological description of the incident progression
- ✅ **Customer Reply** — polished, empathetic, copy-ready customer-facing message
- ✅ **Engineering Notes** — technical detail in monospace format for the eng team
- ✅ **Next Steps** — numbered prioritized remediation actions
- ✅ **Copy Buttons** — one-click copy for customer reply and engineering notes

### 🤖 AI Features
- ✅ **Claude Sonnet 4.6** — parses and clusters raw log lines into structured analysis
- ✅ **Device-aware diagnosis** — BrightSign logs differ from Android player logs
- ✅ **Dual-audience output** — customer reply vs engineering notes written for different readers
- ✅ **Pattern clustering** — groups repeated errors even with slightly different timestamps/paths
- ✅ **Root cause chain** — traces symptom → intermediate cause → root cause

### ⚙️ Technical Features
- ✅ **Next.js 15 App Router** — server + client components
- ✅ **TypeScript strict mode** — fully typed analysis and cluster interfaces
- ✅ **Tailwind CSS** — dark green/emerald theme with severity-coded cluster cards

---

## 🏗️ Architecture

```
LogSenseAI/
├── 📁 app/
│   ├── 📄 page.tsx          # Main UI — log input + full diagnostic output
│   ├── 📄 layout.tsx        # Root layout with dark background
│   ├── 📄 globals.css       # Global styles
│   └── 📁 api/
│       └── 📁 analyze/
│           └── 📄 route.ts  # POST /api/analyze — Claude log analyzer
├── 📁 public/               # Static assets
├── 📄 .env.example          # Environment variable template
├── 📄 package.json
└── 📄 README.md
```

---

## 🖥️ UI Overview

| Section | Description |
|---|---|
| **Device Type Chips** | BrightSign / Samsung SSSP / LG webOS / Android / Raspberry Pi / Chrome OS / Windows / Unknown |
| **Log Textarea** | Paste raw log — monospace, resizable |
| **Load Demo** | Fills textarea with SSL + cache failure sample log |
| **Upload File** | Accept .log / .txt files |
| **Line Counter** | Live lines + character count |
| **Analyze Button** | Triggers Claude log analysis |
| **Severity Card** | Critical / High / Medium / Low / Info badge |
| **Error Count** | Red count card |
| **Warning Count** | Yellow count card |
| **Cluster Count** | Total error clusters card |
| **Root Cause** | Plain-English explanation panel |
| **Timeline** | Incident chronology panel |
| **Error Clusters** | Per-cluster: pattern, occurrences, first/last seen, example |
| **Customer Reply** | Copy-ready customer message with Copy button |
| **Engineering Notes** | Monospace technical notes with Copy button |
| **Next Steps** | Numbered prioritized action list |

---

## 🚀 Getting Started

### Prerequisites
- Node.js 18+
- pnpm
- Anthropic API key ([console.anthropic.com](https://console.anthropic.com))

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/SUDARSHANCHAUDHARI/LogSenseAI.git
   cd LogSenseAI
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Set up environment**
   ```bash
   cp .env.example .env.local
   # Edit .env.local and add your ANTHROPIC_API_KEY
   ```

4. **Run dev server**
   ```bash
   pnpm dev
   ```
   Open [http://localhost:3000](http://localhost:3000)

---

## 📜 Scripts

```bash
pnpm dev      # Start development server (Turbopack)
pnpm build    # Production build
pnpm start    # Start production server
pnpm lint     # ESLint check
```

---

## 🔑 Environment Variables

| Variable | Description | Required |
|---|---|---|
| `ANTHROPIC_API_KEY` | Your Anthropic API key | ✅ Yes |

Get your key at [console.anthropic.com](https://console.anthropic.com). Add it to `.env.local` — this file is gitignored and never committed.

---

## 📊 Current Status

| Property | Value |
|---|---|
| **Version** | 1.0.0 |
| **Status** | ✅ MVP Complete |
| **Model** | claude-sonnet-4-6 |
| **Device Types** | 8 (BrightSign, Samsung SSSP, LG webOS, Android, Raspberry Pi, Chrome OS, Windows, Unknown) |
| **Severity Levels** | 5 (Critical, High, Medium, Low, Info) |
| **Output Sections** | 7 (Severity, Clusters, Root Cause, Timeline, Reply, Notes, Next Steps) |

---

## 🛠️ Tech Stack

| Component | Technology |
|---|---|
| **Framework** | Next.js 15 (App Router) |
| **Language** | TypeScript (strict mode) |
| **Styling** | Tailwind CSS |
| **AI** | Claude API — claude-sonnet-4-6 |
| **Package Manager** | pnpm |

---

## 🔒 Security

- `ANTHROPIC_API_KEY` lives in `.env.local` — gitignored, never committed
- `.env.example` contains placeholder values only
- API key sent directly to Anthropic — no intermediate server
- Log content not stored or logged server-side

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/your-feature`)
3. Commit your changes (`git commit -m 'feat: add your feature'`)
4. Push to the branch (`git push origin feat/your-feature`)
5. Open a Pull Request

---

## 📞 Support

- 🐛 Issues: [GitHub Issues](https://github.com/SUDARSHANCHAUDHARI/LogSenseAI/issues)

---

<div align="center">

**Made with ❤️ by [SUDARSHANCHAUDHARI](https://github.com/SUDARSHANCHAUDHARI)**

[⭐ Star this repo](https://github.com/SUDARSHANCHAUDHARI/LogSenseAI) · [🐛 Report Issue](https://github.com/SUDARSHANCHAUDHARI/LogSenseAI/issues)

</div>
