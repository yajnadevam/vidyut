# Test data

## `rv_let_corpus.tsv`

826 present-stem subjunctive (leṬ) forms from the Rigveda, filtered from
Oliver Hellwig's morpho-lexical annotation of the Rigveda.

- **Tag filtered:** `tense_mode = "Present conjunctive (subjunctive)"`
- **Source:** Hellwig, O., Hettrich, H., Modi, A., Pinkal, M. (2018).
  *Multi-layer Annotation of the Rigveda.* LREC 2018.
  Repository: <https://github.com/OliverHellwig/sanskrit>
- **License:** CC BY 4.0

### Columns

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
| `tense_mode` | always `"Present conjunctive (subjunctive)"` for this filtered subset |

### Used by

`tests/integration/rv_let_corpus.rs` — corpus-driven regression test.
The test reads this file, derives each form via vidyut, and asserts the
expected `surface_form` appears in vidyut's output for the corresponding
dhātu, lakāra=Let, person, and number. Failures are listed all at once
in the test output.
