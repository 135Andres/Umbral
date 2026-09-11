"""Sketch only - NOT in the service yet. See open-questions.md Q1."""

BUCKETS = {}

def allow(client_id, limit=60, window=60):
    # token bucket, deliberately naive
    import time
    now = time.time()
    tokens, last = BUCKETS.get(client_id, (limit, now))
    tokens = min(limit, tokens + (now - last) * (limit / window))
    if tokens < 1:
        BUCKETS[client_id] = (tokens, now)
        return False
    BUCKETS[client_id] = (tokens - 1, now)
    return True

# PROBLEM: the monitoring probes come from one address and would be throttled.
# That is exactly the unresolved question.
