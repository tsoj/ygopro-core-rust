//! BIG Win!?
use super::prelude::*;

pub const ID: u32 = 84677654;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change level of all face-up monsters OR lose LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COIN)
    effect::set_category(d, e1, CATEGORY_COIN);
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
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_XYZ)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceLevel(tp)
    let lv = duel::announce_number_range(d, a.tp, None, None, &[]).nil().0;
    // Duel.SetTargetParam(lv)
    duel::set_target_param(d, lv as u32);
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,PLAYER_ALL,2)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, PLAYER_ALL as u8, 2);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local lv=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]);
    let lv = info[0].int();
    // local coin1=Duel.TossCoin(tp,1)
    let coin1 = duel::toss_coin(d, a.tp, 1)[0];
    // local coin2=Duel.TossCoin(1-tp,1)
    let coin2 = duel::toss_coin(d, 1 - a.tp, 1)[0];
    // if coin1==COIN_HEADS and coin2==COIN_HEADS then
    if coin1 == COIN_HEADS as i32 && coin2 == COIN_HEADS as i32 {
        // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    } else if coin1 == COIN_TAILS as i32 && coin2 == COIN_TAILS as i32 {
        // local lp=Duel.GetLP(tp)
        let lp = duel::get_lp(d, a.tp);
        // Duel.SetLP(tp,lp-lv*500)
        duel::set_lp(d, a.tp, lp - (lv * 500) as i32);
    }
}
