# Decisions, 2024

## 2024-08-14 - Real-time updates: polling, not sockets
Author: Ana
We keep the 30 second polling loop instead of moving to websockets. Rationale:
the socket prototype doubled the memory footprint on the small instances and
the client team did not need sub-second updates.
Revisit if p95 responsiveness becomes a complaint from users.

## 2024-09-02 - Storage: local disk, not object storage
Author: Priya
Object storage added a network hop to every read in the prototype. Revisit when
we need multi-region.
