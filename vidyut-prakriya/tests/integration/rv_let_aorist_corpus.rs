//! Diagnostic coverage report for लुङ्-stem leṬ derivations against the RV.
//!
//! Reads `tests/data/rv_let_aorist_corpus.tsv` — 605 forms filtered from
//! Hellwig's morpho-lexical Rigveda annotation (LREC 2018;
//! `tense_mode = "Aorist conj./subj."`). Forms are derived with
//! `Tinanta::builder().lakara(Let).let_stem(LetStem::Lun)`, which routes
//! the stem-build through `add_lun_vikarana` (cli → kṣa/caṅ/aṅ/sic) while
//! keeping the leṬ pratyaya machinery (3.4.94 agama, 3.4.97 i-drop, etc.).
//!
//! ## Why this test is `#[ignore]`d
//!
//! See `rv_let_corpus.rs` for the Pāṇinian-only framing. The aorist-stem
//! corpus is dominated by Vedic-only formations:
//!
//! - a-aorists for roots not on 3.1.55 / 3.1.59 (`śravat`, `sakat`,
//!   `yāsat`, `saghat`);
//! - un-vṛddhied sic-aorist forms (corpus `vakzati` vs Pāṇinian
//!   `vAkzati` from 7.2.3);
//! - reduplicated aorist (caṅ-style) for roots not on 3.1.48;
//! - sandhi/sigmatic peculiarities that Pāṇini's standard luṅ chain
//!   doesn't produce.
//!
//! Vidyut produces the Pāṇinian-licensed forms (kṛ + leṬ-on-luṅ →
//! `karat, karad, karati, karatE, karate`; tan → `tanat`; etc.). The
//! ~60% gap reflects Vedic morphology beyond the grammar's coverage.
//!
//! Top corpus lemmas: kṛ (68), pṛ (42), yam (41), bhū (40), dhā (40), dā
//! (38), man (28), vah (23), riṣ (20), yaj (18).

extern crate test_utils;

use crate::rv_let_helpers::run_corpus_test;
use vidyut_prakriya::args::LetStem;

const TSV: &str = include_str!("../data/rv_let_aorist_corpus.tsv");

#[test]
#[ignore = "Diagnostic only: remaining failures are Vedic archaic forms Pāṇini doesn't license"]
fn rv_let_aorist_corpus_all_forms_match() {
    run_corpus_test(TSV, "लुङ्-stem", Some(LetStem::Lun));
}
