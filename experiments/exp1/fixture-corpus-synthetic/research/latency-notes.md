# Responsiveness notes

What we measured: the service answers typical dashboard reads in 88 ms at the
median and 410 ms at the 95th percentile (see metrics.json for the window).

What we promised the pilot: "the dashboard will feel instant". 410 ms does not
feel instant, which is why the freshness conversation keeps coming back.

Budget: if we move to sockets we can spend less time re-sending the whole
dashboard and more on the first paint.
