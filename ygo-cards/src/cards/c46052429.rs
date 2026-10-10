//! Advanced Ritual Art
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 46052429;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,extrafil=s.extrafil,extraop=s.extraop,matfilter=s.forcedgroup})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            extrafil: Some(Arc::new(extrafil)),
            extraop: Some(Arc::new(extraop)),
            matfilter: Some(ritual::MatFilt::F(Arc::new(forcedgroup))),
            ..Default::default()
        },
    );
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DECKDES);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // return Duel.GetFieldGroup(tp,LOCATION_DECK,0)
    Some(duel::get_field_group(d, a.tp, LOCATION_DECK, 0))
}
/// `s.extraop(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn extraop(d: &mut Duel, mat: &CardSet, _a: &EffArgs, _tc: CardId) {
    // return Duel.SendtoGrave(mat,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)
    duel::send_to_grave(d, mat.clone(), REASON_EFFECT + REASON_MATERIAL + REASON_RITUAL, None, None);
}
/// `s.forcedgroup(c,e,tp)`
fn forcedgroup(d: &mut Duel, c: CardId, _e: EffectId, _tp: u8) -> bool {
    // return c:IsLocation(LOCATION_DECK) and c:IsType(TYPE_NORMAL) and c:IsAbleToGrave()
    card::is_location(d, c, LOCATION_DECK) && card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_grave(d, c, None)
}
