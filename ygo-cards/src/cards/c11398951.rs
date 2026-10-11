//! Machine Angel Absolute Ritual
use super::prelude::*;

pub const ID: u32 = 11398951;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBER_ANGEL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // Ritual.AddProcEqual{handler=c,filter=s.ritualfil,extrafil=s.extrafil,extraop=s.extraop,extratg=s.extratg}
    ritual::add_proc_equal(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| ritualfil(d, c))),
            extrafil: Some(std::sync::Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            extraop: Some(std::sync::Arc::new(|d: &mut Duel, mg: &CardSet, a: &EffArgs, tc: CardId| extraop(d, mg, a, tc))),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
}
// function s.ritualfil(c)
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CYBER_ANGEL) and c:IsRitualMonster()
    card::is_set_card(d, c, &[SET_CYBER_ANGEL as u16]) && card::is_ritual_monster(d, c)
}
// function s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsRace(RACE_WARRIOR|RACE_FAIRY) and c:IsAbleToDeck()
    card::has_level(d, c) && card::is_race(d, c, RACE_WARRIOR | RACE_FAIRY) && card::is_able_to_deck(d, c)
}
// function s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    if a.chk == Some(0) {
        // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_GRAVE,0,nil)
        return Some(duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_GRAVE, 0, NO_EX));
    }
    // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_GRAVE,0,nil):Filter(aux.nvfilter,nil)
    let g = duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    Some(group::filter(d, &g, &|d, c| aux::nvfilter(d, c), NO_EX))
}
// function s.extraop(mg,e,tp,eg,ep,ev,re,r,rp)
fn extraop(d: &mut Duel, mg: &CardSet, _a: &EffArgs, _tc: CardId) {
    let mut mg = mg.clone();
    // local mat2=mg:Filter(Card.IsLocation,nil,LOCATION_GRAVE):Filter(Card.IsRace,nil,RACE_WARRIOR|RACE_FAIRY)
    let in_grave = group::filter(d, &mg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    let mat2 = group::filter(d, &in_grave, &|d, c| card::is_race(d, c, RACE_WARRIOR | RACE_FAIRY), NO_EX);
    // mg:Sub(mat2)
    group::remove_card(&mut mg, &mat2);
    // Duel.ReleaseRitualMaterial(mg)
    duel::release_ritual_material(d, &mg, false);
    // Duel.SendtoDeck(mat2,nil,SEQ_DECKSHUFFLE,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)
    duel::send_to_deck(d, mat2, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT + REASON_MATERIAL + REASON_RITUAL, None);
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
