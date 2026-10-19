#!/usr/bin/env python3
"""Call the Oxide MCP server: python scripts/mcp.py <tool> ['<json args>'] [out.png]

Examples:
  python scripts/mcp.py page
  python scripts/mcp.py click '{"text": "Start"}'
  python scripts/mcp.py invoke '{"command": "home"}'
  python scripts/mcp.py wait_event '{"name": "check:result", "after": 3}'
  python scripts/mcp.py screenshot '{}' shot.png
"""
import base64
import json
import os
import sys
import urllib.request

sys.stdout.reconfigure(encoding="utf-8")

port = os.environ.get("OXIDE_MCP_PORT", "47811")
tool = sys.argv[1]
args = json.loads(sys.argv[2]) if len(sys.argv) > 2 and sys.argv[2] else {}
out = sys.argv[3] if len(sys.argv) > 3 else None
body = json.dumps({
    "jsonrpc": "2.0", "id": 1, "method": "tools/call",
    "params": {"name": tool, "arguments": args},
}).encode()
req = urllib.request.Request(
    f"http://127.0.0.1:{port}/mcp", data=body, headers={"Content-Type": "application/json"}
)
res = json.load(urllib.request.urlopen(req, timeout=60))
if "error" in res:
    print("error:", res["error"])
    sys.exit(1)
r = res["result"]
for c in r.get("content", []):
    if c["type"] == "image" and out:
        with open(out, "wb") as f:
            f.write(base64.b64decode(c["data"]))
        print("saved", out)
    elif c["type"] == "image":
        print(f"<image {len(c['data'])} b64 chars; pass an out.png path to save>")
    else:
        print(c["text"])
sys.exit(1 if r.get("isError") else 0)
