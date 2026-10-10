//! Sand Gambler
use super::prelude::*;

pub const ID: u32 = 50593156;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss 3 coins and destroy monsters on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COIN+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let _g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 3);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c1,c2,c3=Duel.TossCoin(tp,3)
    let coins = duel::toss_coin(d, a.tp, 3);
    // if Duel.CountHeads(c1,c2,c3)==3 then
    if duel::count_heads(&coins) == 3 {
        // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    // elseif Duel.CountTails(c1,c2,c3)==3 then
    } else if duel::count_tails(&coins) == 3 {
        // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
