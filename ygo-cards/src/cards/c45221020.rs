//! Treasure Panda
use super::prelude::*;

pub const ID: u32 = 45221020;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Normal Monster from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToRemoveAsCost(POS_FACEDOWN)
    card::is_spell_trap(d, c) && card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,#sg,false)
    let lvl = sg.len() as u32;
    (duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e.nil(), tp, lvl, false)), tp, LOCATION_DECK, 0, 1, NO_EX), false)
}
/// `s.spfilter(c,e,tp,lvl,chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lvl: u32, chk: bool) -> bool {
    // if not (c:IsType(TYPE_NORMAL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_type(d, c, TYPE_NORMAL) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    // return (chk and c:IsLevelBelow(lvl)) or c:IsLevel(lvl)
    (chk && card::is_level_below(d, c, lvl)) || card::is_level(d, c, &[lvl])
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=math.min(#g,3)
    let ct = g.len().min(3);
    // if chk==0 then return ct>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ct,true) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ct as u32, true)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,ct,s.rescon,1,tp,HINTMSG_REMOVE,s.rescon)
    let rg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(1),
        Some(ct),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_REMOVE as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.Remove(rg,POS_FACEDOWN,REASON_COST)
    duel::remove(d, &rg, POS_FACEDOWN as u8, REASON_COST, None, None);
    // e:SetLabel(#rg)
    effect::set_label(d, a.e, &[rg.len() as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local lvl=e:GetLabel()
    let lvl = effect::get_label(d, a.e) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,lvl,false)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, lvl, false)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
