# API draft

GET  /health
GET  /items
POST /items
GET  /events        (not implemented; depends on ADR-007)

Auth: static token for the pilot. Not good enough for a real deployment, and
we have not designed the real thing.

If we move to Nimbus the base URL and the token handling do not change, which
was one of the reasons it looked attractive.
