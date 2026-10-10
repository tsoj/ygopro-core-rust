//! Broken Line
use super::prelude::*;

pub const ID: u32 = 88086137;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local seq,p,loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_SEQUENCE,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_SEQUENCE, CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION]);
    let mut seq = info[0].int();
    let p = info[1].int();
    let loc = info[2].int();
    // if p==1-tp then seq=seq+16 end
    if p == 1 - tp as i64 {
        seq += 16;
    }
    // return ((re:IsMonsterEffect() and loc&LOCATION_MZONE>0 and bit.extract(c:GetColumnZone(LOCATION_MZONE),seq)~=0)
    //     or (re:IsHasType(EFFECT_TYPE_ACTIVATE) and loc&LOCATION_SZONE>0 and bit.extract(c:GetColumnZone(LOCATION_SZONE)>>8,seq)~=0))
    //     and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    let part1 = effect::is_monster_effect(d, re)
        && (loc & LOCATION_MZONE as i64) > 0
        && ((card::get_column_zone(d, c, LOCATION_MZONE, 0, 0, None) as i64) >> seq) & 1 != 0;
    let part2 = effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && (loc & LOCATION_SZONE as i64) > 0
        && (((card::get_column_zone(d, c, LOCATION_SZONE, 0, 0, None) as i64) >> 8) >> seq) & 1 != 0;
    (part1 || part2) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
