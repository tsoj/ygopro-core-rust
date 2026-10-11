//! Remote Revenge
use super::prelude::*;

pub const ID: u32 = 61656650;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.efcon)
    effect::set_condition(d, e1, Some(condition(efcon)));
    // e1:SetTarget(s.eftg)
    effect::set_target(d, e1, Some(cost_target(eftg)));
    // e1:SetOperation(s.efop)
    effect::set_operation(d, e1, Some(operation(efop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efcon(e,tp,eg,ep,ev,re,r,rp)`
fn efcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ev = a.ev as u8;
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    // if not g or #g~=1 then return false end
    let Some(g) = duel::get_chain_info(d, ev, &[CHAININFO_TARGET_CARDS as u32])[0].as_group().cloned() else {
        return false;
    };
    if g.len() != 1 {
        return false;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local ex,dg=Duel.GetOperationInfo(0,CATEGORY_DESTROY)
    let dg = duel::get_operation_info(d, 0, CATEGORY_DESTROY).and_then(|op| op.op_cards);
    // return tc:IsControler(tp) and dg and #dg==1 and dg:GetFirst()==tc and tc:IsMonster()
    if !card::is_controler(d, tc, tp) {
        return false;
    }
    match dg {
        Some(dg) => dg.len() == 1 && group::get_first(d, &dg) == Some(tc) && card::is_monster(d, tc),
        None => false,
    }
}
/// `s.filter(c,ct)`
fn filter(d: &mut Duel, c: CardId, ct: u32) -> bool {
    // return Duel.CheckChainTarget(ct,c)
    duel::check_chain_target(d, ct as u8, c)
}
/// `s.eftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ev = a.ev;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,ev) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, ev)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,ev)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, ev)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    let ev = a.ev as u8;
    // local tf=re:GetTarget()
    let _tf = effect::get_target(d, a.re.nil());
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and Duel.CheckChainTarget(ev,tc) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && duel::check_chain_target(d, ev, tc) {
            // local g=Group.FromCards(tc)
            let g = CardSet::from_card(tc);
            // Duel.ChangeTargetCard(ev,g)
            duel::change_target_card(d, ev, &g);
        }
    }
}
