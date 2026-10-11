//! Contact Out
use super::prelude::*;

pub const ID: u32 = 69270537;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEOS as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_NEOS) and c:IsType(TYPE_FUSION) and c:IsAbleToExtra()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NEOS as u16]) && card::is_type(d, c, TYPE_FUSION) && card::is_able_to_extra(d, c)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp,fc)`: `fc.material and c:IsCode(table.unpack(fc.material)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fc: CardId) -> bool {
    let material = crate::class::material(d, fc);
    !material.is_empty() && card::is_code(d, c, &material) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>1 and tc:IsLocation(LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && card::is_location(d, tc, LOCATION_EXTRA)
    {
        // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,tc)
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, tc)), tp, LOCATION_DECK, 0, NO_EX);
        // if tc:CheckFusionMaterial(sg,nil,PLAYER_NONE|FUSPROC_NOTFUSION) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if card::check_fusion_material(d, tc, Some(&sg), None, Some((PLAYER_NONE | FUSPROC_NOTFUSION) as u64))
            && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local mats=Duel.SelectFusionMaterial(tp,tc,sg)
            let mats = duel::select_fusion_material(d, tp, tc, &sg, None, None);
            // Duel.SpecialSummon(mats,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, mats, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
