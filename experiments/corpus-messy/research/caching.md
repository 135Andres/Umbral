# Caching notes

Current: an in-process dict, no eviction (see bug-1422).
Considered: an on-disk cache. Rejected for now - the deployment is a single
small instance and the disk is slow.
Open: whether the cache should be per-tenant. Not decided.
