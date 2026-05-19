//! Corpus-driven regression test for leṬ (present-stem subjunctive) derivations.
//!
//! Reads `tests/data/rv_let_corpus.tsv` — 826 forms filtered from Hellwig's
//! morpho-lexical Rigveda annotation (LREC 2018) where
//! `tense_mode = "Present conjunctive (subjunctive)"`. For each form, looks
//! up the dhātu(s) in vidyut's dhātupāṭha by lemma, derives the leṬ form
//! for the corpus-given person and number, and asserts the corpus
//! `surface_form` is in vidyut's output set.
//!
//! Failures are collected and reported all at once. The test passes only
//! when 100% of the testable forms match. As leṬ-related bugs are fixed in
//! the engine, the failure count should monotonically decrease until empty.
//!
//! Forms whose lemma cannot be located in vidyut's dhātupāṭha (causatives,
//! intensives, denominatives, and a few other derived stems) are excluded
//! from the testable set rather than counted as failures — they live in a
//! reported "matcher gap" bucket.

extern crate test_utils;

use std::collections::HashMap;
use vidyut_prakriya::args::*;
use vidyut_prakriya::dhatupatha::Dhatupatha;
use vidyut_prakriya::Vyakarana;

const TSV: &str = include_str!("../data/rv_let_corpus.tsv");

fn iast_to_slp1(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if next == Some('h') {
            let asp = match c {
                'k' => Some('K'), 'g' => Some('G'),
                'c' => Some('C'), 'j' => Some('J'),
                'ṭ' => Some('W'), 'ḍ' => Some('Q'),
                't' => Some('T'), 'd' => Some('D'),
                'p' => Some('P'), 'b' => Some('B'),
                _ => None,
            };
            if let Some(a) = asp {
                out.push(a);
                i += 2;
                continue;
            }
        }
        let m: &str = match c {
            'ā' => "A", 'ī' => "I", 'ū' => "U",
            'ṛ' => "f", 'ṝ' => "F", 'ḷ' => "x", 'ḹ' => "X",
            'ṅ' => "N", 'ñ' => "Y",
            'ṭ' => "w", 'ḍ' => "q", 'ṇ' => "R",
            'ś' => "S", 'ṣ' => "z", 'ṃ' => "M", 'ḥ' => "H",
            'a' => {
                if next == Some('i') { i += 1; "E" }
                else if next == Some('u') { i += 1; "O" }
                else { "a" }
            }
            _ => { out.push(c); i += 1; continue; }
        };
        out.push_str(m);
        i += 1;
    }
    out
}

fn clean_upadesha(s: &str) -> String {
    let stripped: String = s.chars()
        .filter(|c| !matches!(*c, '~' | '^' | '\\' | '`' | '!'))
        .collect();
    stripped.strip_prefix("qu").unwrap_or(&stripped).to_string()
}

fn match_keys(upadesha: &str) -> Vec<String> {
    let cleaned = clean_upadesha(upadesha);
    let mut keys = std::collections::HashSet::new();
    keys.insert(cleaned.clone());

    let stripped_cons: String = cleaned
        .trim_end_matches(|c: char| {
            matches!(c, 'Y' | 'N' | 'M' | 'K' | 'G' | 'C' | 'J' | 'W' | 'Q' | 'P' | 'B' | 'R' | 'z' | 'S')
        })
        .to_string();
    if stripped_cons != cleaned {
        keys.insert(stripped_cons.clone());
    }

    for base in [cleaned.clone(), stripped_cons.clone()] {
        if base.len() > 1 {
            let last = base.chars().last().unwrap();
            if matches!(last, 'a' | 'i' | 'u' | 'I' | 'U' | 'f' | 'F') {
                let without = base[..base.len() - last.len_utf8()].to_string();
                if !without.is_empty() {
                    keys.insert(without);
                }
            }
        }
    }

    keys.into_iter().collect()
}

fn purusha_from(s: &str) -> Option<Purusha> {
    match s {
        "3" => Some(Purusha::Prathama),
        "2" => Some(Purusha::Madhyama),
        "1" => Some(Purusha::Uttama),
        _ => None,
    }
}

fn vacana_from(s: &str) -> Option<Vacana> {
    match s {
        "1" => Some(Vacana::Eka),
        "2" => Some(Vacana::Dvi),
        "3" => Some(Vacana::Bahu),
        _ => None,
    }
}

#[test]
fn rv_let_corpus_all_forms_match() {
    // Load vidyut's dhātupāṭha and index by all plausible lemma-cleaning variants.
    let dp_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/dhatupatha.tsv");
    let dp = Dhatupatha::from_path(&dp_path).expect("dhatupatha load");
    let mut by_lemma: HashMap<String, Vec<Dhatu>> = HashMap::new();
    for entry in dp.iter() {
        let upad = entry.dhatu().aupadeshika()
            .map(|s| s.to_string()).unwrap_or_default();
        for key in match_keys(&upad) {
            by_lemma.entry(key).or_default().push(entry.dhatu().clone());
        }
    }

    let v = Vyakarana::new();
    let mut lines = TSV.lines();
    lines.next(); // header

    let mut total = 0;
    let mut matched = 0;
    let mut matcher_gap = 0;
    let mut failures: Vec<(String, String, String, String, String, Vec<String>)> = vec![];

    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 10 { continue; }
        let cit = fields[0];
        let surface_iast = fields[3];
        let lemma_iast = fields[4];
        let p_str = fields[7];
        let n_str = fields[8];

        let purusha = match purusha_from(p_str) { Some(x) => x, None => continue };
        let vacana = match vacana_from(n_str) { Some(x) => x, None => continue };
        let surface_slp = iast_to_slp1(surface_iast);
        let lemma_slp = iast_to_slp1(lemma_iast);

        total += 1;

        let candidates = match by_lemma.get(&lemma_slp) {
            Some(v) => v.clone(),
            None => { matcher_gap += 1; continue; }
        };

        let mut all_outputs: Vec<String> = vec![];
        let mut found = false;
        for dhatu in &candidates {
            let tin = match Tinanta::builder()
                .dhatu(dhatu.clone())
                .lakara(Lakara::Let)
                .prayoga(Prayoga::Kartari)
                .purusha(purusha)
                .vacana(vacana)
                .build()
            {
                Ok(t) => t,
                Err(_) => continue,
            };
            for p in v.derive_tinantas(&tin) {
                let t = p.text();
                if t == surface_slp { found = true; }
                all_outputs.push(t);
            }
        }

        if found {
            matched += 1;
        } else {
            all_outputs.sort();
            all_outputs.dedup();
            failures.push((
                cit.into(), surface_slp, lemma_slp,
                p_str.into(), n_str.into(), all_outputs,
            ));
        }
    }

    let testable = total - matcher_gap;
    let report_lines: Vec<String> = failures.iter()
        .map(|(cit, surf, lem, p, n, out)| {
            let out_str = if out.is_empty() { "(empty)".into() } else { out.join(", ") };
            format!("  {:<12} expected={:<22} lemma={:<10} p={} n={} got=[{}]",
                cit, surf, lem, p, n, out_str)
        })
        .collect();

    if !failures.is_empty() {
        // Limit to first 30 lines to keep CI log manageable; print count.
        let preview = report_lines.iter().take(30).cloned().collect::<Vec<_>>().join("\n");
        panic!(
            "\n\nRV leṬ corpus: {matched}/{testable} testable forms match ({:.1}%) — \
             {} failing forms; {matcher_gap} forms skipped due to matcher gap; \
             {total} total in corpus.\n\nFirst 30 of {} failures:\n{}\n\n\
             (Full list available by running examples/validate_let_corpus)\n",
            100.0 * matched as f64 / testable as f64,
            failures.len(), failures.len(), preview
        );
    }
}
