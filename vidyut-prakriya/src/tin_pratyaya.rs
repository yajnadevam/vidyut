//! tin_pratyaya ============
//! (3.4.77 - end of 3.4)
//!
//! The rules in this section have two main functions:
//!
//! 1. Replace a giving lakāra with the appropriate tiṅ-pratyaya. This is called tiṅ-ādeśa ("verb
//!    ending substitution"). To perform tiṅ-ādeśa, we must know the puruṣa, vacana, and pada
//!    associated with this prakriyā.
//!
//! 2. Modify the basic tiṅ-pratyaya according to the lakāra and any other conditions relevant to
//!    the prakriyā (for example, vidhi-liṅ vs. āśīr-liṅ). This is called tiṅ-siddhi ("verb ending
//!    completion").
//!
//! All of these rules are found at the end of section 3.4 of the Ashtadhyayi.

use crate::args::{
    Agama as A, Aupadeshika as Au, DhatuPada, Gana, Lakara, Purusha, Tin, Vacana, Vikarana as V,
};
use crate::core::operators as op;
use crate::core::{Code, Morph, Prakriya, PrakriyaTag as PT, Rule, Tag as T};
use crate::it_samjna;
use crate::misc::uses_sip_vikarana;

const TIN_PARA: &[&str] = &["tip", "tas", "Ji", "sip", "Tas", "Ta", "mip", "vas", "mas"];
const NAL_PARA: &[&str] = &["Ral", "atus", "us", "Tal", "aTus", "a", "Ral", "va", "ma"];

/// Replaces the lakAra with a tiN-pratyaya.
pub fn adesha(p: &mut Prakriya, purusha: Purusha, vacana: Vacana) {
    let pada = if p.has_tag(PT::Parasmaipada) {
        DhatuPada::Parasmaipada
    } else {
        assert!(p.has_tag(PT::Atmanepada));
        DhatuPada::Atmanepada
    };
    let tin = Tin::from_args(pada, purusha, vacana);

    if let Some(i) = p.find_last_with_tag(T::Pratyaya) {
        p.set(i, |t| {
            t.add_tags(&[
                // 1.4.104
                T::Vibhakti,
                T::Tin,
                purusha.as_tag(),
                vacana.as_tag(),
                pada.as_tag(),
            ]);
            t.morph = Morph::Tin(tin);
        });
        op::adesha("3.4.78", p, i, tin.as_str());

        // Ignore Nit-tva that we get from the lakAra. Kashika on 3.4.103:
        //
        //   lakArAzrayaGitvam AdezAnAM na bhavati.
        //
        // Likewise, this rule ignores the N of mahiN, which is just for the sake
        // of making a pratyAhAra.
        if p.has(i, |t| t.has_tag(T::Nit)) {
            p.set(i, |t| t.remove_tag(T::Nit));
        }
    }
}

fn yatha(rule: Code, p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    p.run(rule, |p| op::upadesha_yatha(p, i, old, new));
    it_samjna::run(p, i).ok();
}

fn yatha_optional(rule: Code, p: &mut Prakriya, i: usize, old: &[&str], new: &[&str]) {
    if p.optional_run(rule, |p| op::upadesha_yatha(p, i, old, new)) {
        it_samjna::run(p, i).ok();
    }
}

fn siddhi(p: &mut Prakriya, la: Lakara) -> Option<()> {
    use Lakara::*;

    let i_dhatu = p.find_last_with_tag(T::Dhatu)?;
    let i = p.find_last_with_tag(T::Tin)?;

    // Special case: handle lut_siddhi first.
    let tin = p.get(i)?;
    if tin.has_tag(T::Prathama) && la == Lakara::Lut {
        let ending = if tin.has_tag(T::Ekavacana) {
            "qA"
        } else if tin.has_tag(T::Dvivacana) {
            "rO"
        } else {
            "ras"
        };
        op::adesha("2.4.85", p, i, ending);
        return None;
    }

    let tin = p.get(i)?;
    let la = tin.lakara?;

    if tin.is_atmanepada() && !la.is_nit() {
        let ta_jha = &["ta", "Ja"];
        let es_irec = &["eS", "irec"];
        if tin.has_lakara(Lit) && tin.has_text_in(ta_jha) {
            yatha("3.4.81", p, i, ta_jha, es_irec);
        } else if tin.is(Tin::TAs) {
            op::adesha("3.4.80", p, i, "se");
        } else {
            p.run_at("3.4.79", i, op::ti("e"));
        }
    }

    let dhatu = p.get(i_dhatu)?;
    let tin = p.get(i)?;
    if tin.has_lakara(Lit) && tin.is_parasmaipada() {
        yatha("3.4.82", p, i, TIN_PARA, NAL_PARA);
    } else if tin.has_lakara(Lat) && tin.is_parasmaipada() {
        if dhatu.is_u(Au::vida_2) && tin.has_u_in(TIN_PARA) {
            yatha_optional("3.4.83", p, i, TIN_PARA, NAL_PARA);
        } else if dhatu.has_text("brU") && tin.has_u_in(&TIN_PARA[..5]) {
            let ran = p.optional_run("3.4.84", |p| {
                p.set(i_dhatu, |t| t.set_text("Ah"));
                op::upadesha_yatha(p, i, TIN_PARA, NAL_PARA);
            });
            if ran {
                it_samjna::run(p, i).ok();
            }
        }
    } else if tin.has_lakara(Let) {
        // 3.4.89 also fires under leṬ — mip → ni in 1sg (parallel to its
        // application under Loṭ below). Yields bhavāni in the pit branch.
        if tin.ends_with("mi") {
            op::adesha("3.4.89", p, i, "ni");
        }

        // 3.4.94 lewo'qAwO — leṬ takes aṭ or āṭ. We pick based on the
        // stem-final phonology so the agama produces the right surface
        // form without relying on a+a sandhi (which doesn't always fire
        // cleanly across stem+vikaraṇa+agama boundaries):
        //   - stem ends in 'a' (most class 1, 6, 10): āṭ would duplicate
        //     the contraction (bhava+ā=bhavā), so use aṭ which becomes
        //     long via 6.1.101 a+a sandhi. Either gives `bhavāti`.
        //   - stem ends in consonant (class 2, 5, 7, etc.): use aṭ
        //     directly; no sandhi to lengthen — matches corpus (e.g.,
        //     `asat`, not `asāt`).
        // The earlier `uses_sip_vikarana` carve-out is preserved for
        // juzī~/tF/madi~ where vidyut adds the sip-vikaraṇa (3.1.34).
        let prev_text = p
            .terms()
            .iter()
            .take(i)
            .rev()
            .find(|t| !t.text.is_empty())
            .map(|t| t.text.clone())
            .unwrap_or_default();
        let stem_ends_in_a = prev_text.ends_with('a');
        // For leṬ-on-luṄ-stem (Phase 4), always use aṭ — the corpus
        // consistently shows the short-a agama (karat, śravat, sakat).
        // 6.1.101 (a+a → ā) handles the a-final-stem cases (manāmahe).
        let is_lun_stem = p.has_tag(PT::FlagLetStemLun);
        // For leṬ-on-liṬ-stem (Phase 5), 3.4.94 leaves the choice between
        // aṭ and āṭ open. The reduplicated perfect stem is consonant-final,
        // so 6.1.101 sandhi never lengthens — only āṭ gives the long-ā
        // forms (papṛcāsi). Try both branches.
        let is_lit_stem = p.has_tag(PT::FlagLetStemLit);
        let agama = if is_lun_stem || uses_sip_vikarana(p, i_dhatu) || !stem_ends_in_a {
            A::aw
        } else {
            A::Aw
        };

        // Special case: when sip-vikaraṇa was applied AND the pratyaya is
        // sip (2sg paras), the modal-a-agama is optionally omitted. This
        // gives the Vedic 2sg paras forms like yakzi, vakzi (vs. agama-inserted
        // yakzasi). 8.2.36 + 8.2.41 + sandhi fire on the no-agama branch giving
        // the corpus form. NOTE: this is not licensed by any Pāṇinian sūtra or
        // vārttika — Kāśikā 3.4.94 says the aṭ/āṭ agama comes obligatorily
        // (paryāyeṇa). The RV tags these as subjunctive, but Macdonell (Vedic
        // Grammar §451, p.336) lists yakzi (yaj), vakzi (vah) and Srozi (Sru) in
        // one paradigm as root-class present 2sg "with imperative sense" — which
        // is also why the optional-sip set in vikarana.rs is exactly yaj/vah/Sru.
        // We cite that attestation via Rule::Anyatra rather than a fabricated code.
        let has_sip_vikarana = p.terms().iter().any(|t| t.is(crate::args::Vikarana::sip));
        let skip_agama = has_sip_vikarana && p.has(i, |t| t.is(Tin::sip));

        let mut agama_inserted = true;
        if skip_agama {
            // Optional no-agama branch.
            let skipped = p.optional_run(Rule::Anyatra("leT 2sg paras sib-vikarana with no aT/AT agama -- yakzi (yaj) vakzi (vah) Srozi (Sru); RV tags subjunctive but Macdonell Vedic Grammar 451 p336 lists these as root-class present 2sg with imperative sense -- RV 1.75.5 1.188.3,https://archive.org/details/cu31924023050325/page/336"), |p| {
                p.set(i, |t| t.add_tag(T::pit));
            });
            if skipped {
                agama_inserted = false;
            }
        }
        if agama_inserted {
            // For leṬ-on-liṬ-stem: 3.4.94 leaves aṭ/āṭ open. The default
            // `agama` value above is aṭ for the consonant-final reduplicated
            // stem; here we also try āṭ as an optional branch so the long-ā
            // forms (papṛcāsi, jujozāt) are reachable.
            if is_lit_stem && agama == A::aw {
                let inserted_long = p.optional_run("3.4.94", |p| {
                    p.set(i, |t| t.add_tag(T::pit));
                    p.insert(i, A::Aw);
                });
                if !inserted_long {
                    p.run("3.4.94", |p| {
                        p.set(i, |t| t.add_tag(T::pit));
                        p.insert(i, A::aw);
                    });
                }
            } else {
                p.run("3.4.94", |p| {
                    p.set(i, |t| t.add_tag(T::pit));
                    p.insert(i, agama);
                });
            }
            it_samjna::run(p, i).ok()?;
        }

        let i = if agama_inserted { i + 1 } else { i };
        let tin = p.get(i)?;
        if tin.has_adi('A') {
            // mantrayEte, mantrayETe ,...
            p.run_at("3.4.95", i, op::adi("E"));
        } else if p.has(i_dhatu, |t| t.has_u_in(&["juzI~", "tF", "madi~"]))
            && tin.is_parasmaipada()
            && tin.has_antya('i')
        {
            // jozizat, tArizat (3.4.97 is mandatory for these dhātus).
            p.run_at("3.4.97", i, op::antya_lopa);
        } else if p.has_tag(PT::Uttama) && tin.has_antya('s') {
            // karavAva, karavAma; karavAvaH, karavAmaH. Covers the vārttika
            // *leṭsambandhin uttamapuruṣasya sakārasya vā lopo bhavati*.
            p.optional_run_at("3.4.98", i, op::antya_lopa);
        }

        // 3.4.96 (vaito'nyatra) for leṬ ātmanepada: e → ai (E in SLP1),
        // optionally (vā). Kāśikā: leṭsambandhina ekārasya vaikārādeśo bhavati.
        // Corpus: anaśāmahai/bravāvahai/kṛṇavai (Uttama), yajātai (3sg).
        //
        // Excluded: non-Uttama atm DUAL cells, which already get the dual
        // -aite (= -Ete in SLP1) form via 3.4.95 ata→ai (anyatra). Adding a
        // further e→ai there would give the overgenerated -EtE form (e.g.,
        // mantrayEtE) which isn't corpus-attested — Kāśikā likewise blocks it
        // (anyatreti kim? mantrayaite).
        let tin = p.get(i)?;
        let exclude_non_uttama_dual = tin.has_tag(T::Dvivacana) && !p.has_tag(PT::Uttama);
        if tin.is_atmanepada() && tin.has_antya('e') && !exclude_non_uttama_dual {
            p.optional_run_at("3.4.96", i, op::antya("E"));
        }

        // 3.4.97 (itaśca lopaḥ parasmaipadeṣu): under leṬ, the final -i of any
        // parasmaipada pratyaya optionally drops. Kāśikā: leṭsambandhina ikārasya
        // parasmaipadaviṣayasya lopo bhavati; vānuvṛtteḥ pakṣe śravaṇam api bhavati
        // (the vā carries down from 3.4.96). Produces the short forms BavAt,
        // BavAn, BavAs alongside the retained BavAti, BavAnti, BavAsi. This
        // replaces the earlier Phase 1 T::Nit-fork modeling (which was
        // non-Pāṇinian and triggered a spurious 1.2.4 cascade producing forms
        // like kurvEte that are not corpus-attested).
        //
        // The 1sg Uttama is excluded because the lakāra-derived ṅit-tva
        // does not transfer to the -ni substitute from 3.4.89 (Kāśikā on
        // 3.4.103: lakArAzrayaGitvam AdezAnAM na bhavati). So the -i of -ni
        // is not droppable under this path. The bare-ā 1sg comes from the
        // separate Anyatra-cited Vedic form below.
        let tin = p.get(i)?;
        if tin.is_parasmaipada()
            && tin.has_antya('i')
            && !(tin.has_tag(T::Uttama) && tin.has_tag(T::Ekavacana))
        {
            p.optional_run_at("3.4.97", i, op::antya_lopa);
        }

        // Bare-ā 1sg subjunctive: under leṬ Uttama 1sg paras, the pratyaya
        // optionally luk-elides entirely, yielding kṛṇavā, bharā, bravā, stavā,
        // arcā, ayā, etc. (~19 forms in the RV). For bhū this gives bhavā
        // alongside bhavāni. This is not from any Pāṇinian sūtra (the leṬ sūtras
        // yield bhavāt via 3.4.97 and bhavāni via 3.4.89, not the endingless
        // -ā); it is the Vedic 1sg subjunctive in -ā documented by Macdonell,
        // Vedic Grammar §471 p347 (kṛṇavā, hinavā). Cited via Rule::Anyatra.
        let tin = p.get(i)?;
        if p.has_tag(PT::Uttama) && tin.has_tag(T::Ekavacana) && tin.is_parasmaipada() {
            p.optional_run_at(Rule::Anyatra("leT uttamapurusa ekavacana bare-A subjunctive (RV ~19 forms) -- kfRavA hinavA bravA stavA arcA; Macdonell Vedic Grammar 471 p347 -- RV 10.95.2 10.39.5 2.11.6,https://archive.org/details/cu31924023050325/page/347"), i, op::lopa);
        }

        // Note: Vedic pādānta lengthening (final -a → -ā at line-end) is
        // attested in ~6 corpus forms (kṛṇavāmā, carātā, etc.) but is
        // meter-driven, not phonological. Generating it as a free alternate
        // produces unattested forms like `bhavātā` 2pl paras that conflict
        // with the manual kashika test expectations. Deferred to a future
        // phase that handles meter context.
    } else if tin.has_lakara(Lot) {
        // Applies tin-siddhi rules that apply to just loT.
        if tin.is(Tin::sip) {
            p.run_at("3.4.87", i, |t| {
                t.set_u("hi");
                t.set_text("hi");
                t.remove_tag(T::pit);
            });

            if p.is_chandasi() {
                p.optional_run_at("3.4.88", i, op::add_tag(T::Pit));
            }
        } else if tin.ends_with("mi") {
            // BavAni
            op::adesha("3.4.89", p, i, "ni");
        } else if tin.has_antya('i') {
            // Bavatu
            p.run_at("3.4.86", i, op::antya("u"));
        } else if tin.has_antya('e') {
            if tin.has_tag(T::Uttama) && tin.has_antya('e') {
                // karavE, karavAvahE, karavAmahE
                p.run_at("3.4.93", i, op::antya("E"));
            } else if tin.ends_with("se") || tin.ends_with("ve") {
                // pacasva, pacaDvam
                p.run_at("3.4.91", i, |t| {
                    let n = t.len();
                    if t.ends_with("se") {
                        t.text.replace_range(n - 2.., "sva");
                    } else {
                        t.text.replace_range(n - 2.., "vam");
                    }
                });
            } else {
                // pacatAm, pacetAm, pacantAm
                p.run_at("3.4.90", i, op::antya("Am"));
            }
        }

        let tin = p.get(i)?;
        if tin.has_tag(T::Uttama) {
            // BavAni
            p.run("3.4.92", |p| {
                // Add pit to the pratyaya, not the Agama.
                p.set(i, |t| t.add_tag(T::pit));
                p.insert(i, A::Aw);
            });
            it_samjna::run(p, i).ok()?;
        }
    }

    // Substitution of jhi --> jus.
    //
    // Notes:
    // - must occur before 3.4.100 when performing loT/nit siddhi.
    let tin = p.get(i)?;
    if tin.is(Tin::Ji) {
        if matches!(la, Lakara::AshirLin | Lakara::VidhiLin) {
            op::adesha("3.4.108", p, i, "jus");
        } else if la.is_nit() {
            let i_prev = p.prev_not_empty(i)?;
            let prev = p.get(i_prev)?;

            let is_vid = prev.has_text("vid") && prev.has_gana(Gana::Adadi);
            if prev.is(V::sic) || prev.has_tag(T::Abhyasta) || is_vid {
                op::adesha("3.4.109", p, i, "jus");
            } else if prev.is_dhatu() {
                if prev.has_antya('A') {
                    if la == Lakara::Lan {
                        op::optional_adesha("3.4.111", p, i, "jus");
                    } else {
                        op::adesha("3.4.110", p, i, "jus");
                    }
                } else if prev.has_text("dviz") && la == Lakara::Lan {
                    op::optional_adesha("3.4.112", p, i, "jus");
                }
            }
        }
    }

    // lot and Nit siddhi.
    // (Includes lo~w by 3.4.85)
    let i = p.find_last_with_tag(T::Tin)?;
    if la == Lakara::Lot || la.is_nit() {
        let tas_thas = &["tas", "Tas", "Ta", "mip"];
        let taam_tam = &["tAm", "tam", "ta", "am"];
        if p.has(i, |t| t.has_u_in(tas_thas)) {
            yatha("3.4.101", p, i, tas_thas, taam_tam);
        }

        if p.has(i, |t| t.is_parasmaipada()) {
            if p.has(i, |t| t.has_tag(T::Uttama) && t.has_antya('s')) {
                p.run_at("3.4.99", i, op::antya(""));
            }

            // lo~w excluded by existence of 3.4.86
            if p.has(i, |t| t.has_antya('i')) && la != Lakara::Lot {
                p.run_at("3.4.100", i, op::antya(""));
            }
        }
    }

    // liN-only siddhi
    if p.has(i, |t| t.is_lin_lakara()) {
        if p.has(i, |t| t.is_parasmaipada()) {
            p.insert(i, A::yAsuw);
            if la == Lakara::AshirLin {
                // ucyAt
                // Add kit to the pratyaya, not the Agama.
                p.add_tag_at("3.4.104", i + 1, T::kit);
            } else {
                // kuryAt
                // Add Nit to the pratyaya, not the Agama.
                p.add_tag_at("3.4.103", i + 1, T::Nit);
            }
            it_samjna::run(p, i).expect("agama");
        } else {
            // paceta; pakzIzwa
            p.insert(i, A::sIyuw);
            p.step("3.4.102");
            it_samjna::run(p, i).expect("agama");

            let i_tin = i + 1;
            if p.has(i_tin, |t| t.is(Tin::Ja)) {
                // paceran, yajeran, kfzIran
                op::adesha("3.4.105", p, i_tin, "ran");
            } else if p.has(i_tin, |t| t.is(Tin::iw)) {
                // paceya, yajeya, kfzIya, hfzIya
                op::adesha("3.4.106", p, i_tin, "a");
            }
        }

        let i = p.find_last_with_tag(T::Tin)?;
        if p.has(i, |t| t.text.contains('t') || t.text.contains('T')) {
            // kfzIzwa, kfzIyAstAm; kfzIzWAH, kfzIyAsTAm
            p.set(i, |t| {
                t.find_and_replace_text("t", "st");
                t.find_and_replace_text("T", "sT");
            });
            p.step("3.4.107");
        }
    }

    // The 'S' of 'eS' is just for sarva-Adeza (1.1.55). If it is kept, it will
    // cause many problems when deriving li~T. So, remove it here.
    let i = p.find_last_with_tag(T::Tin)?;
    if p.has(i, |t| t.has_u("eS")) {
        p.set(i, |t| t.remove_tag(T::Sit));
    }

    Some(())
}

/// Applies substitutions to the given tin suffix.
pub fn try_general_siddhi(p: &mut Prakriya, la: Lakara) -> Option<()> {
    if !p.terms().last()?.is(Tin::Ji) {
        siddhi(p, la);
    }
    Some(())
}

/// Applies substitutions to the given tin suffix.
///
/// Due to rule 3.4.109 ("sic-abhyasta-vidibhyaH ca"), this should run after dvitva and the
/// insertion of vikaraNas.
pub fn try_siddhi_for_jhi(p: &mut Prakriya, la: Lakara) -> Option<()> {
    if p.terms().last()?.is(Tin::Ji) {
        siddhi(p, la);
    }
    Some(())
}
