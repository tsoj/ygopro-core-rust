//! Equip Shot
use super::prelude::*;

pub const ID: u32 = 62878208;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase()
    duel::is_battle_phase(d, None)
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local ec=c:GetEquipTarget()
    // return ec and ec:IsControler(tp) and ec:IsPosition(POS_FACEUP_ATTACK)
    //     and Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil,c)
    let Some(ec) = card::get_equip_target(d, c) else {
        return false;
    };
    card::is_controler(d, ec, tp)
        && card::is_position(d, ec, POS_FACEUP_ATTACK)
        && duel::is_existing_target(d, Some(&|d, x| filter2(d, x, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.filter2(c,ec)`
fn filter2(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and ec:CheckEquipTarget(c)
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::check_equip_target(d, ec, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil,e,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first1 = group::get_first(d, &g1).nil();
    effect::set_label_object(d, e, Value::Card(first1));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUPATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUPATTACK as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil,g1:GetFirst())
    duel::select_target(d, tp, Some(&|d, c| filter2(d, c, first1)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g1), 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local eq=e:GetLabelObject()
    let eq = effect::get_label_object(d, e).as_card().nil();
    // local eqc=eq:GetEquipTarget()
    let eqc = card::get_equip_target(d, eq);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS as u32])[0].as_group().cloned().nil();
    // local tc=g:GetFirst()
    // if eq==tc then tc=g:GetNext() end
    let order = group::iter(d, &g);
    let mut tc = order.first().copied();
    if tc == Some(eq) {
        tc = order.get(1).copied();
    }
    // if eqc and eq:IsRelateToEffect(e) then
    if let Some(eqc) = eqc {
        if card::is_relate_to_effect(d, eq, e) {
            let tc = tc.nil();
            // if Duel.Equip(tp,eq,tc)==0 then return end
            // (Duel.Equip returns a boolean, which is never equal to 0)
            duel::equip(d, tp, eq, tc, true, false);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local a=eqc
            // local d=tc
            let mut attacker = eqc;
            let mut defender = tc;
            // if Duel.IsTurnPlayer(1-tp) then
            if duel::is_turn_player(d, 1 - tp) {
                // a=tc
                // d=eqc
                attacker = tc;
                defender = eqc;
            }
            // if a:CanAttack() and not a:IsImmuneToEffect(e) and not d:IsImmuneToEffect(e) then
            if card::can_attack(d, attacker) && !card::is_immune_to_effect(d, attacker, e) && !card::is_immune_to_effect(d, defender, e) {
                // Duel.CalculateDamage(a,d,true)
                duel::calculate_damage(d, attacker, Some(defender), true);
            }
        }
    }
}
