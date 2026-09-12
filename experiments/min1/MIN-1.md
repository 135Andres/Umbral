# MIN-1 — field ablation over the minimum record model

| | |
|---|---|
| **Purpose** | test whether the fields proposed for a minimum record model are load-bearing, or merely plausible |
| **Question** | if a field is removed from the model, does some required interpretation become impossible? |
| **Hypotheses** | the model M = {Entity, Assertion, Relation, Provenance, Time} supplied by the research mandate is minimal (treated as a hypothesis under attack, not a finding) |
| **Method** | a throwaway fixture of ~13 small scenarios; for each candidate field, ablate it and attempt the interpretation the field was supposed to carry. `ablation.py` is stdlib-only and prints which fields survive |
| **Evidence** | `ablation.py`, `run1.log` (raw output) |
| **Result** | every field in the fixture turned out load-bearing, including `scope` and the pair of times — two of which are **not** primitives of the proposed five. The fixture's own first version failed to discriminate subject and time; that defect was found and fixed before the result was accepted |
| **Limitations** | **declared circularity**: the fixtures were written by the same process that proposed the fields, so the ablation shows internal consistency, not independent confirmation. The scenarios are small and authored, not sampled from real projects |
| **Conclusion** | the five-primitive model is not minimal as stated; the record-level reduction it supported is recorded as a hypothesis in `research/PROJECT-REALITY-MINIMUM-MODEL.md` (H24). This experiment cannot settle the question on its own |
| **Status** | RESULT RECORDED (2026-09-11); the circularity is the reason the next experiment (E-MIN-1, field ablation with fresh readers) is proposed rather than a larger fixture |

This is a throwaway research fixture, not product code. It is not part of any Umbral
architecture, and it selected nothing.

Related: `research/PROJECT-REALITY-MINIMUM-MODEL.md` (the research artifact this experiment
was built to falsify), `research/scratch-minimum-model-scenarios.md` (working notes, not
public material).
