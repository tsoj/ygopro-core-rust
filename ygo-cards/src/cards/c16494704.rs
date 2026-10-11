//! Odd-Eyes Advent
//! オッドアイズ・アドベント
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 16494704;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_GREATER,filter=s.ritualfil,extrafil=s.extrafil,extraop=s.extraop,matfilter=s.forcedgroup,location=LOCATION_HAND|LOCATION_GRAVE})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| ritualfil(d, x))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            extraop: Some(Arc::new(|d: &mut Duel, mg: &CardSet, a: &EffArgs, tc: CardId| extraop(d, mg, a, tc))),
            matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, x: CardId, e: EffectId, tp: u8| forcedgroup(d, x, e, tp)))),
            location: Some(LOCATION_HAND | LOCATION_GRAVE),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ritualfil(c)`
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON)
    card::is_race(d, c, RACE_DRAGON as u64)
}
/// `s.exfilter0(c)`
fn exfilter0(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ODD_EYES) and c:IsLevelAbove(1) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_ODD_EYES as u16]) && card::is_level_above(d, c, 1) && card::is_able_to_grave(d, c, None)
}
/// `s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>1 then
    if duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 1 {
        // return Duel.GetMatchingGroup(s.exfilter0,tp,LOCATION_EXTRA,0,nil)
        return Some(duel::get_matching_group(d, Some(&exfilter0), tp, LOCATION_EXTRA, 0, NO_EX));
    }
    None
}
/// `s.extraop(mg,e,tp,eg,ep,ev,re,r,rp)`
fn extraop(d: &mut Duel, mg: &CardSet, _a: &EffArgs, _tc: CardId) {
    let mut mg = mg.clone();
    // local mat2=mg:Filter(Card.IsLocation,nil,LOCATION_EXTRA)
    let mat2 = group::filter(d, &mg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX);
    // mg:Sub(mat2)
    group::remove_card(&mut mg, &mat2);
    // Duel.ReleaseRitualMaterial(mg)
    duel::release_ritual_material(d, &mg, false);
    // Duel.SendtoGrave(mat2,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)
    duel::send_to_grave(d, mat2, REASON_EFFECT + REASON_MATERIAL + REASON_RITUAL, None, None);
}
/// `s.forcedgroup(c,e,tp)`
fn forcedgroup(d: &mut Duel, c: CardId, _e: EffectId, _tp: u8) -> bool {
    // return (c:IsType(TYPE_PENDULUM) and c:IsLocation(LOCATION_HAND|LOCATION_ONFIELD)) or (c:IsSetCard(SET_ODD_EYES) and c:IsLocation(LOCATION_EXTRA))
    (card::is_type(d, c, TYPE_PENDULUM) && card::is_location(d, c, LOCATION_HAND | LOCATION_ONFIELD))
        || (card::is_set_card(d, c, &[SET_ODD_EYES as u16]) && card::is_location(d, c, LOCATION_EXTRA))
}
