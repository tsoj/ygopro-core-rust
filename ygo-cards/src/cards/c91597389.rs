//! Roll Out!
use super::prelude::*;

pub const ID: u32 = 91597389;
pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_UNION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_UNION) and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_MZONE,0,1,nil,c)
    card::is_type(d, c, TYPE_UNION) && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.eqfilter(c,ec)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsFaceup() and ec:CheckUnionTarget(c) and aux.CheckUnionEquip(ec,c)
    card::is_faceup(d, c) && card::check_union_target(d, ec, c) && aux::check_union_equip(d, ec, c)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc, a.tp);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_MZONE,0,1,1,nil,tc):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter(d, c, tc)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // if Duel.Equip(tp,tc,sc) then
    if duel::equip(d, a.tp, tc, sc, true, false) {
        // aux.SetUnionState(tc)
        aux::set_union_state(d, tc);
    }
}
