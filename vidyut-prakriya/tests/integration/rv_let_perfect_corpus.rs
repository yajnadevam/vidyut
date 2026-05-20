//! Diagnostic coverage report for लिट्-stem leṬ derivations against the RV.
//!
//! Reads `tests/data/rv_let_perfect_corpus.tsv` — 182 forms filtered from
//! Hellwig's morpho-lexical Rigveda annotation (LREC 2018;
//! `tense_mode = "Perfect conjunctive (subj.)"`). Derived with
//! `Tinanta::builder().lakara(Let).let_stem(LetStem::Lit)`, which skips
//! vikaraṇa and triggers liṬ-style dvitva (6.1.8) on the dhātu, then runs
//! the leṬ pratyaya machinery (3.4.94 agama optional aṭ/āṭ, 3.4.97
//! i-drop, etc.) on top of the reduplicated stem.
//!
//! ## Why this test is `#[ignore]`d
//!
//! See `rv_let_corpus.rs` for the Pāṇinian-only framing. The perfect-stem
//! corpus has dense Vedic-only material:
//!
//! - long-ā ("intensive") reduplication (`rāraṇat`, `vāvṛdhāti`,
//!   `cākanaḥ`) — Pāṇini's 7.4.59 shortens reduplicants;
//! - strong-stem guṇa under sārvadhātuka (`ciketat`, `jujozat`,
//!   `mAmahaH`, `daDarzati`) — 7.3.87 blocks guṇa for abhyasta +
//!   piti-sārvadhātuka + ac-initial pratyaya;
//! - optative-on-perfect-stem (`baBUyAt`) — different formation
//!   (liṅ + Lit), not leṬ-on-Lit.
//!
//! Vidyut produces the Pāṇinian-licensed forms (juṣ → `jujuzat,
//! jujuzati`; pṛc → `papṛcāsi`; cit → `cicitat`). The ~63% gap reflects
//! Vedic perfect-stem-subjunctive morphology beyond the grammar.
//!
//! Top corpus lemmas: juṣ (20), cit (17), pyā (16), prath (10), tan (9),
//! sūd (8), dāś (8), kan (7), vṛt (7).

extern crate test_utils;

use crate::rv_let_helpers::run_corpus_test;
use vidyut_prakriya::args::LetStem;

const TSV: &str = include_str!("../data/rv_let_perfect_corpus.tsv");

#[test]
#[ignore = "Diagnostic only: remaining failures are Vedic archaic forms Pāṇini doesn't license"]
fn rv_let_perfect_corpus_all_forms_match() {
    run_corpus_test(TSV, "लिट्-stem", Some(LetStem::Lit));
}
