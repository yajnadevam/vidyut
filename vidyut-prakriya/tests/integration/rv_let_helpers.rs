//! Shared helpers for the three RV leṬ corpus regression tests
//! (`rv_let_corpus`, `rv_let_aorist_corpus`, `rv_let_perfect_corpus`).
//!
//! Each test loads a TSV of corpus forms filtered by Hellwig's
//! morpho-lexical `tense_mode` tag, then uses the helpers here to
//! convert IAST→SLP1, find candidate dhātus by lemma, and report
//! match results.

use std::collections::HashMap;
use vidyut_prakriya::args::*;
use vidyut_prakriya::dhatupatha::Dhatupatha;
use vidyut_prakriya::Vyakarana;

pub fn iast_to_slp1(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if next == Some('h') {
            let asp = match c {
                'k' => Some('K'),
                'g' => Some('G'),
                'c' => Some('C'),
                'j' => Some('J'),
                'ṭ' => Some('W'),
                'ḍ' => Some('Q'),
                't' => Some('T'),
                'd' => Some('D'),
                'p' => Some('P'),
                'b' => Some('B'),
                _ => None,
            };
            if let Some(a) = asp {
                out.push(a);
                i += 2;
                continue;
            }
        }
        let m: &str = match c {
            'ā' => "A",
            'ī' => "I",
            'ū' => "U",
            'ṛ' => "f",
            'ṝ' => "F",
            'ḷ' => "x",
            'ḹ' => "X",
            'ṅ' => "N",
            'ñ' => "Y",
            'ṭ' => "w",
            'ḍ' => "q",
            'ṇ' => "R",
            'ś' => "S",
            'ṣ' => "z",
            'ṃ' => "M",
            'ḥ' => "H",
            'a' => {
                if next == Some('i') {
                    i += 1;
                    "E"
                } else if next == Some('u') {
                    i += 1;
                    "O"
                } else {
                    "a"
                }
            }
            _ => {
                out.push(c);
                i += 1;
                continue;
            }
        };
        out.push_str(m);
        i += 1;
    }
    out
}

fn clean_upadesha(s: &str) -> String {
    let stripped: String = s
        .chars()
        .filter(|c| !matches!(*c, '~' | '^' | '\\' | '`' | '!'))
        .collect();
    // Strip Pāṇinian prefix-anubandhas so dhātupāṭha keys line up with the
    // bare-root lemmas the corpus uses:
    //   - `qu` allows the krt-pratyaya `Tuc` per 3.1.90 (`qukf\Y` → kf).
    //   - `wu` allows the krt-pratyaya `aTuc` per 3.1.89. Some `wu`-prefixed
    //     entries also carry an `o` separator before the actual root letters
    //     (`wuo~Svi` → `wuoSvi`, where the root is `Svi`), so we strip the
    //     `o` ONLY when `wu` was stripped (`o`-initial roots like `oKf~`,
    //     `oRf~`, `olaqi~` must keep their leading `o`).
    if let Some(rest) = stripped.strip_prefix("qu") {
        return rest.to_string();
    }
    if let Some(rest) = stripped.strip_prefix("wu") {
        return rest.strip_prefix('o').unwrap_or(rest).to_string();
    }
    stripped
}

pub fn match_keys(upadesha: &str) -> Vec<String> {
    let cleaned = clean_upadesha(upadesha);
    let mut keys = std::collections::HashSet::new();

    let mut bases: Vec<String> = vec![cleaned.clone()];
    let trim_cons = |s: &str| -> String {
        s.trim_end_matches(|c: char| {
            matches!(
                c,
                'Y' | 'N'
                    | 'M'
                    | 'K'
                    | 'G'
                    | 'C'
                    | 'J'
                    | 'W'
                    | 'Q'
                    | 'P'
                    | 'B'
                    | 'R'
                    | 'z'
                    | 'S'
                    | 'r'
                    | 'l'
                    | 'h'
            )
        })
        .to_string()
    };
    let mut last_base = cleaned.clone();
    loop {
        let s1 = trim_cons(&last_base);
        if s1 == last_base {
            break;
        }
        bases.push(s1.clone());
        if let Some(last) = s1.chars().last() {
            if matches!(last, 'i' | 'I' | 'u' | 'U' | 'a') && s1.len() > 1 {
                let without = s1[..s1.len() - last.len_utf8()].to_string();
                if !without.is_empty() {
                    bases.push(without.clone());
                    last_base = without;
                    continue;
                }
            }
        }
        last_base = s1;
    }

    // Strip trailing vowel-anubandhas. `f`/`F`/`x`/`X` are vocalic ṛ/ṝ/ḷ/ḹ
    // and are commonly used as marker anubandhas at the end of upadeśas
    // (e.g. `ga\mx~` → `gam`, where `x` is the `xit` anubandha per the
    // luṅ-aṅ rule 3.1.55). a/i/u/I/U also appear as anubandha-like markers
    // in some entries.
    let mut more: Vec<String> = vec![];
    for base in &bases {
        if base.len() > 1 {
            let last = base.chars().last().unwrap();
            if matches!(last, 'a' | 'i' | 'u' | 'I' | 'U' | 'f' | 'F' | 'x' | 'X') {
                let without = base[..base.len() - last.len_utf8()].to_string();
                if !without.is_empty() {
                    more.push(without);
                }
            }
        }
    }
    bases.extend(more);

    // Surface-form normalizations applied to every base.
    //
    // The corpus lemma uses the **surface** spelling of the root, while
    // Pāṇini's dhātupāṭha uses an artificial **upadeśa** spelling. We close
    // the gap by adding the surface variant alongside the upadeśa form:
    //
    //   - retroflex → dental swaps that always apply to root-initial
    //     consonants by Pāṇinian convention:
    //         R → n  (8.4.41 etc.; `Ra\Sa~`  → naś)
    //         z → s  (6.1.64;     `zWA\`    → sthā, used together with W→T)
    //         W → T  (combined with z→s,    `zWA\` → sTA — retroflex aspirated
    //                 to dental aspirated stop)
    //         Q → D, q → d, w → t — analogous retroflex→dental swaps
    //   - palatal-nasal alternation: in upadeśa the dental `n` is used before
    //     palatal stops c/j/J/Y (e.g. `anjU~` for añj); the surface form has
    //     palatal `Y` (`aYj`).
    let mut normalized: Vec<String> = vec![];
    for base in &bases {
        let has_retro = base.contains(['R', 'z', 'W', 'Q', 'q', 'w']);
        if has_retro {
            let surf: String = base
                .chars()
                .map(|c| match c {
                    'R' => 'n',
                    'z' => 's',
                    'W' => 'T',
                    'Q' => 'D',
                    'q' => 'd',
                    'w' => 't',
                    _ => c,
                })
                .collect();
            normalized.push(surf);
        }
        // NOTE: do NOT normalize F → f / X → x. The short and long
        // vocalic vowels distinguish distinct Pāṇinian roots (e.g.
        // `mf` = mṛ "to die" vs `mF` = mṝ "to kill"; `kf` = kṛ vs
        // `kF` = kṝ). Conflating them would let mismatching dhātus
        // sneak into the candidate set.
        // Palatal nasal: in upadeśa, `n` before c/j/J/Y is dental; the surface
        // root spells it palatal `Y`.
        let bytes = base.as_bytes();
        let mut palatalized = String::with_capacity(base.len());
        let mut changed = false;
        for i in 0..bytes.len() {
            let c = bytes[i] as char;
            let next = bytes.get(i + 1).map(|b| *b as char);
            if c == 'n' && matches!(next, Some('c') | Some('j') | Some('J') | Some('Y')) {
                palatalized.push('Y');
                changed = true;
            } else {
                palatalized.push(c);
            }
        }
        if changed {
            normalized.push(palatalized);
        }
    }
    bases.extend(normalized);

    for b in bases {
        keys.insert(b);
    }
    keys.into_iter().collect()
}

pub fn purusha_from(s: &str) -> Option<Purusha> {
    match s {
        "3" => Some(Purusha::Prathama),
        "2" => Some(Purusha::Madhyama),
        "1" => Some(Purusha::Uttama),
        _ => None,
    }
}

pub fn vacana_from(s: &str) -> Option<Vacana> {
    match s {
        "1" => Some(Vacana::Eka),
        "2" => Some(Vacana::Dvi),
        "3" => Some(Vacana::Bahu),
        _ => None,
    }
}

/// Build a lemma→Dhatu index from vidyut's dhātupāṭha.
pub fn build_lemma_index() -> HashMap<String, Vec<Dhatu>> {
    let dp_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/dhatupatha.tsv");
    let dp = Dhatupatha::from_path(&dp_path).expect("dhatupatha load");
    let mut by_lemma: HashMap<String, Vec<Dhatu>> = HashMap::new();
    for entry in dp.iter() {
        let upad = entry
            .dhatu()
            .aupadeshika()
            .map(|s| s.to_string())
            .unwrap_or_default();
        for key in match_keys(&upad) {
            by_lemma.entry(key).or_default().push(entry.dhatu().clone());
        }
    }
    by_lemma
}

/// Run a corpus regression test for a given TSV.
///
/// `description` is shown in the panic message (e.g., "लट्-stem",
/// "लुङ्-stem", "लिट्-stem"). `stem` selects which leṬ stem to derive
/// against — `None` for default (लट्-stem), `Some(LetStem::Lun)` for
/// aorist-stem, `Some(LetStem::Lit)` for perfect-stem.
pub fn run_corpus_test(tsv: &str, description: &str, stem: Option<LetStem>) {
    let by_lemma = build_lemma_index();
    // All leṬ derivations are inherently Vedic. Several Pāṇinian rules that
    // produce attested RV forms (e.g. 3.1.59 *kṛ/mṛ/dṝ/ruh + aN* in chandasi)
    // are gated on the chandasi flag, so we enable it for these tests.
    let v = Vyakarana::builder().is_chandasi(true).build();
    let mut lines = tsv.lines();
    lines.next(); // header

    let mut total = 0;
    let mut matched = 0;
    let mut matcher_gap = 0;
    let mut failures: Vec<(String, String, String, String, String, Vec<String>)> = vec![];

    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 10 {
            continue;
        }
        let cit = fields[0];
        let surface_iast = fields[3];
        let lemma_iast = fields[4];
        let root_iast = fields[5];
        let p_str = fields[7];
        let n_str = fields[8];

        let Some(purusha) = purusha_from(p_str) else {
            continue;
        };
        let Some(vacana) = vacana_from(n_str) else {
            continue;
        };
        let surface_slp = iast_to_slp1(surface_iast);
        let lemma_slp = iast_to_slp1(lemma_iast);
        let root_slp = iast_to_slp1(root_iast);

        total += 1;

        // Candidate dhātus and whether we should apply explicit Ṇic.
        //
        // Hellwig's annotation gives both `lemma` (column 5, the surface
        // dictionary stem) and `root` (column 6, the underlying dhātu). For
        // ṇic-derived stems the lemma is the -ay- stem (e.g. `mṛḍay`,
        // `coday`) while the root is the base (`mṛḍ`, `cud`). If the lemma
        // alone doesn't resolve, also try the root with an explicit
        // `Sanadi::Ric` so that Curādi auto-ṇic / explicit-ṇic forms
        // (`codayāti` from `cuda~` etc.) are reachable.
        let mut candidates: Vec<(Dhatu, bool)> = vec![];
        if let Some(v) = by_lemma.get(&lemma_slp) {
            candidates.extend(v.iter().map(|d| (d.clone(), false)));
        }
        if !root_slp.is_empty() && root_slp != lemma_slp {
            if let Some(v) = by_lemma.get(&root_slp) {
                candidates.extend(v.iter().map(|d| (d.clone(), true)));
            }
        }
        if candidates.is_empty() {
            matcher_gap += 1;
            continue;
        }

        let mut all_outputs: Vec<String> = vec![];
        let mut found = false;
        for (dhatu, with_nic) in &candidates {
            let dhatu = if *with_nic {
                dhatu.clone().with_sanadi(&[Sanadi::Ric])
            } else {
                dhatu.clone()
            };
            let mut builder = Tinanta::builder()
                .dhatu(dhatu)
                .lakara(Lakara::Let)
                .prayoga(Prayoga::Kartari)
                .purusha(purusha)
                .vacana(vacana);
            if let Some(s) = stem {
                builder = builder.let_stem(s);
            }
            let tin = match builder.build() {
                Ok(t) => t,
                Err(_) => continue,
            };
            for p in v.derive_tinantas(&tin) {
                let t = p.text();
                if t == surface_slp {
                    found = true;
                }
                all_outputs.push(t);
            }
        }

        if found {
            matched += 1;
        } else {
            all_outputs.sort();
            all_outputs.dedup();
            failures.push((
                cit.into(),
                surface_slp,
                lemma_slp,
                p_str.into(),
                n_str.into(),
                all_outputs,
            ));
        }
    }

    let testable = total - matcher_gap;
    let report_lines: Vec<String> = failures
        .iter()
        .map(|(cit, surf, lem, p, n, out)| {
            let out_str = if out.is_empty() {
                "(empty)".into()
            } else {
                out.join(", ")
            };
            format!(
                "  {:<12} expected={:<22} lemma={:<10} p={} n={} got=[{}]",
                cit, surf, lem, p, n, out_str
            )
        })
        .collect();

    if !failures.is_empty() {
        let preview = report_lines
            .iter()
            .take(30)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");
        let pct = if testable == 0 {
            0.0
        } else {
            100.0 * matched as f64 / testable as f64
        };
        panic!(
            "\n\nRV leṬ corpus ({description}): {matched}/{testable} testable forms match \
             ({pct:.1}%) — {} failing forms; {matcher_gap} forms skipped due to matcher \
             gap; {total} total in corpus.\n\nFirst 30 of {} failures:\n{}\n\n\
             (Full list available by running examples/validate_let_corpus)\n",
            failures.len(),
            failures.len(),
            preview
        );
    }
}
