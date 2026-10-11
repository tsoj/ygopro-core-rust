//! Ancient Gear Factory
use super::prelude::*;

pub const ID: u32 = 30435145;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_GEAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,g)`
fn filter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return c:GetLevel()>4 and c:IsSetCard(SET_ANCIENT_GEAR) and g:CheckWithSumEqual(Card.GetLevel,c:GetLevel()*2,1,99)
    let lv = card::get_level(d, c);
    lv > 4
        && card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16])
        && group::check_with_sum_equal(d, g, &|d, c| card::get_level(d, c) as i64, lv * 2, 1, 99).0
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsSetCard(SET_ANCIENT_GEAR) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::has_level(d, c) && card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local rg=Duel.GetMatchingGroup(s.rfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
        let rg = duel::get_matching_group(d, Some(&rfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,rg)
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, &rg)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local rg=Duel.GetMatchingGroup(s.rfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&rfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,rg)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, &rg)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg=rg:SelectWithSumEqual(tp,Card.GetLevel,tc:GetLevel()*2,1,99)
        let tlv = card::get_level(d, tc);
        let sg = group::select_with_sum_equal(d, &rg, a.tp, &|d, c| card::get_level(d, c) as i64, tlv * 2, 1, 99).nil();
        // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SUMMON_PROC)
        effect::set_code(d, e1, EFFECT_SUMMON_PROC);
        // e1:SetCondition(s.ntcon)
        effect::set_condition(d, e1, Some(lua_fn(ntcon)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0
        && card::get_level(d, c) > 4
        && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
