//! Skyforce Monk
//! Monk of the Tenyi
use super::prelude::*;

pub const ID: u32 = 32519092;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link summon method
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 1, Some(1), None, None, None);
}
/// `s.matfilter(c,lc,stype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_TENYI,lc,stype,tp) and not c:IsType(TYPE_LINK,lc,stype,tp)
    card::is_set_card_ctx(d, c, &[SET_TENYI as u16], lc, stype, Some(tp)) && !card::is_type_ctx(d, c, TYPE_LINK, lc, stype, Some(tp))
}
