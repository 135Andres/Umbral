# Deploy checklist (draft)

Owner: Priya
Status: blocked

1. build the service
2. run the smoke tests
3. copy the artefact to the host
4. restart the supervisor
5. verify the health endpoint

BLOCKED: step 5 cannot be finished until the rate limiting question in
open-questions.md is resolved - the health endpoint is the one that gets
hammered by the monitors, and we have not decided whether to exempt it.
