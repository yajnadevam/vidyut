# Test data

## `rv_let_corpus.tsv` — present-stem leṬ (लट्-stem)

826 present-stem subjunctive (leṬ) forms from the Rigveda, filtered from
Oliver Hellwig's morpho-lexical annotation of the Rigveda.

- **Tag filtered:** `tense_mode = "Present conjunctive (subjunctive)"`

## `rv_let_aorist_corpus.tsv` — aorist-stem leṬ (लुङ्-stem)

605 aorist-stem subjunctive forms from the Rigveda. Used for the Phase 4
implementation track (leṬ built on the aorist stem, distinct from the
present-stem leṬ in `rv_let_corpus.tsv`).

- **Tag filtered:** `tense_mode = "Aorist conj./subj."`

## `rv_let_perfect_corpus.tsv` — perfect-stem leṬ (लिट्-stem)

182 perfect-stem subjunctive forms from the Rigveda. Used for the Phase 5
implementation track (leṬ built on the perfect/reduplicated stem).

- **Tag filtered:** `tense_mode = "Perfect conjunctive (subj.)"`

## Source and license

- **Source:** Hellwig, O., Hettrich, H., Modi, A., Pinkal, M. (2018).
  *Multi-layer Annotation of the Rigveda.* LREC 2018.
  Repository: <https://github.com/OliverHellwig/sanskrit>
- **License:** CC BY 4.0

### Columns (same for all three TSVs)

| Column | Meaning |
|---|---|
| `ref` | maṇḍala.sūkta.ṛc.pāda (e.g., `1.166.14.2` = RV 1.166.14, line 2 = pādas c+d) |
| `pos` | word position within the line |
| `word` | surface form (IAST) |
| `surface_form` | canonical surface form (IAST, same as `word` for most rows) |
| `lemma` | dictionary lemma (IAST) |
| `root`, `prefixes` | usually empty |
| `person` | 1 / 2 / 3 |
| `number` | 1 (sg) / 2 (du) / 3 (pl) |
| `tense_mode` | matches the per-TSV filter above |

### Used by

- `tests/integration/rv_let_corpus.rs` — लट्-stem (active baseline, ~83% match)
- `tests/integration/rv_let_aorist_corpus.rs` — लुङ्-stem (Phase 4 scaffold; currently expected to fail until `Tinanta::let_stem(LetStem::Lun)` is implemented)
- `tests/integration/rv_let_perfect_corpus.rs` — लिट्-stem (Phase 5 scaffold; currently expected to fail until `Tinanta::let_stem(LetStem::Lit)` is implemented)

Each test reads its TSV, derives each form via vidyut, and asserts the
corpus `surface_form` appears in vidyut's output. Failures are listed all
at once in the test output.
