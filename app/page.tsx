"use client";

import { useState, useRef } from "react";

type Severity = "critical" | "high" | "medium" | "low" | "info";

interface Cluster {
  type: "error" | "warning" | "info";
  pattern: string;
  occurrences: number;
  firstSeen: string;
  lastSeen: string;
  example: string;
}

interface Analysis {
  severity: Severity;
  errorCount: number;
  warningCount: number;
  clusters: Cluster[];
  rootCause: string;
  timeline: string;
  customerReply: string;
  engineeringNotes: string;
  nextSteps: string[];
}

const SEVERITY_STYLES: Record<Severity, string> = {
  critical: "bg-red-500/20 text-red-300 border-red-500/30",
  high: "bg-orange-500/20 text-orange-300 border-orange-500/30",
  medium: "bg-yellow-500/20 text-yellow-300 border-yellow-500/30",
  low: "bg-blue-500/20 text-blue-300 border-blue-500/30",
  info: "bg-gray-500/20 text-gray-300 border-gray-500/30",
};

const CLUSTER_STYLES = {
  error: "border-l-red-500 bg-red-500/5",
  warning: "border-l-yellow-500 bg-yellow-500/5",
  info: "border-l-blue-500 bg-blue-500/5",
};

const DEVICES = ["BrightSign", "Samsung SSSP", "LG webOS", "Android", "Raspberry Pi", "Chrome OS", "Windows", "Unknown"];

const DEMO_LOGS = `2026-05-17 09:00:01 [INFO] Player started. Version 3.2.1
2026-05-17 09:00:03 [INFO] Connecting to CMS: https://api.screencloud.com
2026-05-17 09:00:05 [ERROR] SSL certificate verification failed: certificate has expired
2026-05-17 09:00:05 [ERROR] Connection refused: https://api.screencloud.com (SSL handshake failure)
2026-05-17 09:00:06 [WARN] Falling back to cached content
2026-05-17 09:00:06 [INFO] Loading cached playlist: playlist_v3.json (age: 47h)
2026-05-17 09:01:12 [ERROR] SSL certificate verification failed: certificate has expired
2026-05-17 09:01:12 [ERROR] Retry 1/3 failed
2026-05-17 09:02:12 [ERROR] SSL certificate verification failed: certificate has expired
2026-05-17 09:02:12 [ERROR] Retry 2/3 failed
2026-05-17 09:03:12 [ERROR] SSL certificate verification failed: certificate has expired
2026-05-17 09:03:12 [ERROR] Retry 3/3 failed. Giving up.
2026-05-17 09:03:13 [WARN] Cache is 47 hours old. Content may be stale.
2026-05-17 09:03:13 [INFO] Playing cached content (stale)
2026-05-17 09:15:00 [ERROR] Asset not found: https://cdn.screencloud.com/assets/summer_promo.mp4
2026-05-17 09:15:00 [ERROR] HTTP 403 Forbidden
2026-05-17 09:15:01 [WARN] Skipping asset, displaying placeholder
2026-05-17 09:30:00 [ERROR] SSL certificate verification failed: certificate has expired
2026-05-17 09:30:00 [ERROR] Sync attempt failed`;

function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);
  return (
    <button onClick={() => { navigator.clipboard.writeText(text); setCopied(true); setTimeout(() => setCopied(false), 2000); }}
      className="text-xs px-2.5 py-1.5 rounded-lg bg-white/10 hover:bg-white/20 transition-colors text-white/50 hover:text-white">
      {copied ? "✓ Copied" : "Copy"}
    </button>
  );
}

export default function Home() {
  const [logs, setLogs] = useState("");
  const [deviceType, setDeviceType] = useState("Unknown");
  const [loading, setLoading] = useState(false);
  const [analysis, setAnalysis] = useState<Analysis | null>(null);
  const [error, setError] = useState("");
  const fileRef = useRef<HTMLInputElement>(null);

  function onFile(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = ev => setLogs(String(ev.target?.result || ""));
    reader.readAsText(file);
  }

  async function analyze() {
    if (!logs.trim()) return;
    setLoading(true); setError(""); setAnalysis(null);
    try {
      const res = await fetch("/api/analyze", {
        method: "POST", headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ logs, deviceType }),
      });
      const data = await res.json();
      if (data.error) throw new Error(data.error);
      setAnalysis(data);
    } catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  }

  return (
    <div className="min-h-screen bg-[#0a0a0f] text-white">
      <div className="border-b border-white/10 px-6 py-4 flex items-center gap-3">
        <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center text-sm font-mono">{">"}_</div>
        <span className="font-semibold text-lg tracking-tight">LogSense AI</span>
        <span className="text-xs text-white/30 ml-1">Log Analyzer & Diagnostics</span>
      </div>

      <div className="max-w-6xl mx-auto px-6 py-10 space-y-6">
        {/* Input */}
        <div className="space-y-4">
          <div className="flex items-center gap-4 flex-wrap">
            <div>
              <label className="text-xs text-white/40 uppercase tracking-wider block mb-1.5">Device Type</label>
              <div className="flex flex-wrap gap-2">
                {DEVICES.map(d => (
                  <button key={d} onClick={() => setDeviceType(d)}
                    className={`px-3 py-1.5 rounded-lg text-sm transition-all ${deviceType === d ? "bg-green-600 text-white" : "bg-white/5 text-white/50 hover:bg-white/10 hover:text-white"}`}>
                    {d}
                  </button>
                ))}
              </div>
            </div>
          </div>

          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-xs text-white/40 uppercase tracking-wider">Log Output</label>
              <div className="flex items-center gap-2">
                <button onClick={() => setLogs(DEMO_LOGS)} className="text-xs text-white/30 hover:text-white transition-colors">Load demo</button>
                <button onClick={() => fileRef.current?.click()} className="text-xs px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-white/50 hover:text-white transition-all">
                  Upload file
                </button>
                <input ref={fileRef} type="file" accept=".log,.txt" className="hidden" onChange={onFile} />
              </div>
            </div>
            <textarea value={logs} onChange={e => setLogs(e.target.value)}
              placeholder="Paste log output here, or upload a .log / .txt file..."
              className="w-full h-48 bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-sm text-white/80 placeholder-white/20 resize-none focus:outline-none focus:border-green-500/50 transition-all font-mono" />
            <div className="text-xs text-white/20 mt-1">{logs.split("\n").filter(Boolean).length} lines · {logs.length} chars</div>
          </div>

          <button onClick={analyze} disabled={loading || !logs.trim()}
            className="w-full py-3.5 rounded-xl font-semibold text-sm bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-500 hover:to-emerald-500 disabled:opacity-40 disabled:cursor-not-allowed transition-all">
            {loading ? (
              <span className="flex items-center justify-center gap-2">
                <svg className="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24"><circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"/><path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"/></svg>
                Analyzing logs...
              </span>
            ) : "Analyze Logs"}
          </button>
          {error && <div className="p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-300 text-sm">{error}</div>}
        </div>

        {/* Results */}
        {analysis && (
          <div className="space-y-5">
            {/* Header stats */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
              <div className={`p-4 rounded-2xl border flex items-center gap-3 ${SEVERITY_STYLES[analysis.severity]}`}>
                <div>
                  <p className="text-xs opacity-60 uppercase tracking-wider">Severity</p>
                  <p className="font-bold capitalize">{analysis.severity}</p>
                </div>
              </div>
              <div className="p-4 rounded-2xl bg-red-500/10 border border-red-500/20">
                <p className="text-xs text-red-300/60 uppercase tracking-wider">Errors</p>
                <p className="text-2xl font-bold text-red-300">{analysis.errorCount}</p>
              </div>
              <div className="p-4 rounded-2xl bg-yellow-500/10 border border-yellow-500/20">
                <p className="text-xs text-yellow-300/60 uppercase tracking-wider">Warnings</p>
                <p className="text-2xl font-bold text-yellow-300">{analysis.warningCount}</p>
              </div>
              <div className="p-4 rounded-2xl bg-white/5 border border-white/10">
                <p className="text-xs text-white/40 uppercase tracking-wider">Clusters</p>
                <p className="text-2xl font-bold text-white">{analysis.clusters.length}</p>
              </div>
            </div>

            {/* Root cause + Timeline */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="p-5 rounded-2xl bg-white/5 border border-white/10 space-y-2">
                <h3 className="text-xs font-semibold text-white/40 uppercase tracking-wider">Root Cause</h3>
                <p className="text-sm text-white/80 leading-relaxed">{analysis.rootCause}</p>
              </div>
              <div className="p-5 rounded-2xl bg-white/5 border border-white/10 space-y-2">
                <h3 className="text-xs font-semibold text-white/40 uppercase tracking-wider">Timeline</h3>
                <p className="text-sm text-white/70 leading-relaxed">{analysis.timeline}</p>
              </div>
            </div>

            {/* Error clusters */}
            <div className="space-y-2">
              <h3 className="text-xs font-semibold text-white/40 uppercase tracking-wider">Error Clusters</h3>
              {analysis.clusters.map((c, i) => (
                <div key={i} className={`p-4 rounded-2xl border-l-2 ${CLUSTER_STYLES[c.type]}`}>
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-1">
                        <span className="text-sm font-medium text-white">{c.pattern}</span>
                        <span className="text-xs px-2 py-0.5 rounded-full bg-white/10 text-white/50">{c.occurrences}×</span>
                      </div>
                      <p className="text-xs font-mono text-white/40 mt-1 truncate">{c.example}</p>
                    </div>
                    <div className="text-right shrink-0">
                      <p className="text-xs text-white/30">First: {c.firstSeen}</p>
                      <p className="text-xs text-white/30">Last: {c.lastSeen}</p>
                    </div>
                  </div>
                </div>
              ))}
            </div>

            {/* Replies + Notes */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="p-5 rounded-2xl bg-green-500/5 border border-green-500/20 space-y-3">
                <div className="flex items-center justify-between">
                  <h3 className="text-xs font-semibold text-green-400 uppercase tracking-wider">Customer Reply</h3>
                  <CopyButton text={analysis.customerReply} />
                </div>
                <p className="text-sm text-white/80 leading-relaxed whitespace-pre-wrap">{analysis.customerReply}</p>
              </div>
              <div className="p-5 rounded-2xl bg-white/5 border border-white/10 space-y-3">
                <div className="flex items-center justify-between">
                  <h3 className="text-xs font-semibold text-white/40 uppercase tracking-wider">Engineering Notes</h3>
                  <CopyButton text={analysis.engineeringNotes} />
                </div>
                <p className="text-sm text-white/60 leading-relaxed font-mono whitespace-pre-wrap">{analysis.engineeringNotes}</p>
              </div>
            </div>

            {/* Next steps */}
            <div className="p-5 rounded-2xl bg-white/5 border border-white/10 space-y-3">
              <h3 className="text-xs font-semibold text-white/40 uppercase tracking-wider">Next Steps</h3>
              <div className="space-y-2">
                {analysis.nextSteps.map((s, i) => (
                  <div key={i} className="flex items-start gap-2 text-sm text-white/70">
                    <span className="text-green-500 font-bold shrink-0">{i + 1}.</span>{s}
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
