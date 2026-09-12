# Pull request

## What changed?

<!-- Files, and what they now say or do. -->

## Why?

<!-- The problem this solves, or the thing it makes possible. -->

## Evidence / tests?

<!--
A link, a measurement, a reproducible command — or an explicit "no evidence, this is
reasoning". Do not present reasoning as evidence.
-->

## Documentation?

<!-- Which canonical document now needs to agree with this change, if any. -->

## Does this change a decision, invariant, principle or hypothesis?

<!--
If yes: say which one, and on whose authority. A contributor cannot add a decision record
on their own authority — propose it instead (see CONTRIBUTING.md).
If no: say "no".
-->

## Does it introduce a new architectural commitment?

<!--
If yes, it needs a proposal and the project owner's decision first. "No" is the expected
answer for most pull requests.
-->

---

<details>
<summary>Checklist</summary>

- [ ] One concern per pull request.
- [ ] No decision was added or altered without the project owner's authority.
- [ ] No hypothesis is presented as a decision, and no research as validation.
- [ ] Historical records were not rewritten.
- [ ] Relative links resolve, and no document claims something the repository contradicts.
- [ ] If `fsp-check/` was touched: `cargo fmt --check`, `cargo clippy --all-targets`,
      `cargo test` all pass.

</details>
