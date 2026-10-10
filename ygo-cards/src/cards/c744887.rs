//! Utgarda, Generaider Boss of Delusion
use super::prelude::*;

pub const ID: u32 = 744887;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsRace(RACE_ROCK) or (c:IsSetCard(SET_GENERAIDER) and c:IsMonster())) and (c:IsControler(tp) or c:IsFaceup())
    (card::is_race(d, c, RACE_ROCK) || (card::is_set_card(d, c, &[SET_GENERAIDER as u16]) && card::is_monster(d, c)))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.check(sg,tp)`
fn check(d: &mut Duel, sg: &CardSet, tp: u8) -> bool {
    // return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,sg)
    duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, sg.into())
}
/// `s.check` as the `check` argument of the Release cost functions (the extra `exg` is not used)
fn release_check(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> (bool, bool) {
    (check(d, sg, tp), false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,s.cfilter,2,false,s.check,nil,tp)
        return aux::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 2, 2, false, Some(&release_check), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,2,2,false,s.check,nil,tp)
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 2, 2, false, Some(&release_check), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let _tc = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Remove(tc,POS_FACEUP,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
