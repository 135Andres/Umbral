import json, http.server, time

CACHE = {}          # unbounded, see bug-1422

class Handler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/health":
            self.send_response(200); self.end_headers(); self.wfile.write(b"ok")
            return
        if self.path == "/items":
            body = json.dumps(CACHE.get("items", [])).encode()
            self.send_response(200); self.end_headers(); self.wfile.write(body)
            return
        self.send_response(404); self.end_headers()
