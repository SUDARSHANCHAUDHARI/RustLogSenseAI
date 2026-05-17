import Anthropic from "@anthropic-ai/sdk";
import { NextRequest, NextResponse } from "next/server";

const client = new Anthropic({ apiKey: process.env.ANTHROPIC_API_KEY });

export async function POST(req: NextRequest) {
  const { logs, deviceType } = await req.json();
  if (!logs?.trim()) return NextResponse.json({ error: "No logs provided" }, { status: 400 });

  const truncated = logs.slice(0, 8000);

  const prompt = `You are a digital signage platform engineer and expert log analyst. Analyze these logs and extract structured intelligence.

Device type: ${deviceType || "Unknown"}

Logs:
${truncated}

Return JSON with exactly this structure:
{
  "severity": "critical" | "high" | "medium" | "low" | "info",
  "errorCount": number,
  "warningCount": number,
  "clusters": [
    {
      "type": "error" | "warning" | "info",
      "pattern": "short description of the error pattern",
      "occurrences": number,
      "firstSeen": "timestamp or line ref",
      "lastSeen": "timestamp or line ref",
      "example": "one example log line"
    }
  ],
  "rootCause": "Clear technical explanation of the primary root cause",
  "timeline": "Brief narrative of what happened and in what order",
  "customerReply": "A clear, jargon-free customer reply explaining the issue and what's being done",
  "engineeringNotes": "Detailed technical notes for the engineering team: what to investigate, which components are involved, suggested fixes",
  "nextSteps": ["step 1", "step 2", "step 3"]
}

Return ONLY valid JSON.`;

  try {
    const message = await client.messages.create({
      model: "claude-sonnet-4-6",
      max_tokens: 1500,
      messages: [{ role: "user", content: prompt }],
    });
    const text = (message.content[0] as { type: string; text: string }).text;
    return NextResponse.json(JSON.parse(text));
  } catch (e) {
    return NextResponse.json({ error: String(e) }, { status: 500 });
  }
}
