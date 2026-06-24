//! Tests anchored on Macdonell, *A Vedic Grammar for Students* (Oxford, 1916).
//!
//! These tests assert full Vedic paradigms as presented by Macdonell, rather
//! than individual Aṣṭādhyāyī sutras. They complement the per-sutra Kāśikā
//! tests by exercising end-to-end derivations and locking in cross-cell
//! expectations (e.g., that all 9 cells of a paradigm match the printed
//! grammar.)
//!
//! References:
//! - Macdonell, A. A. *A Vedic Grammar for Students*. Oxford, 1916.

extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

/// Full bhū parasmaipada paradigm under leṬ — Macdonell §159.
///
/// Locks in Phase 1's full output: 3.4.89 mip→ni under leṬ + 3.4.97
/// (itaśca lopaḥ parasmaipadeṣu, optional) for the long/short alternation +
/// 3.4.98 optional s-lopa for Uttama + the Vedic bare-`ā` 1sg (Macdonell
/// §471, cited via Anyatra). All 9 cells covered.
#[test]
fn bhu_let_paradigm_159() {
    let bhu = d("BU", Bhvadi);

    // 3rd person — optional final -i drop (3.4.97).
    assert_has_tip(&[], &bhu, Let, &["BavAti", "BavAt"]);
    assert_has_tas(&[], &bhu, Let, &["BavAtaH"]);
    assert_has_jhi(&[], &bhu, Let, &["BavAnti", "BavAn"]);

    // 2nd person — same optional final -i drop.
    assert_has_sip(&[], &bhu, Let, &["BavAsi", "BavAH"]);
    assert_has_thas(&[], &bhu, Let, &["BavATaH"]);
    assert_has_tha(&[], &bhu, Let, &["BavATa"]);

    // 1sg — bhavāni (3.4.89 mip→ni) and bhavā (Vedic bare-ā 1sg, Anyatra/Macdonell §471).
    // bhavān is suppressed: lakāra-derived ṅit-tva does not transfer to
    // the -ni substitute (Kāśikā on 3.4.103).
    assert_has_mip(&[], &bhu, Let, &["BavAni", "BavA"]);

    // 1du / 1pl — both s-lopa-applied and s-retained, per the vārttika
    // *leṭsambandhin uttamapuruṣasya sakārasya vā lopo bhavati* (3.4.98).
    assert_has_vas(&[], &bhu, Let, &["BavAva", "BavAvaH"]);
    assert_has_mas(&[], &bhu, Let, &["BavAma", "BavAmaH"]);
}
