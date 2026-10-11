//! Ghostrick-Go-Round
use super::prelude::*;

pub const ID: u32 = 37055344;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // pos
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(condition_fn)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase()
    duel::is_battle_phase(d, None)
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GHOSTRICK) and c:IsCanTurnSet()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GHOSTRICK as u16]) && card::is_can_turn_set(d, c)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE)
    card::is_position(d, c, POS_FACEDOWN_DEFENSE)
}
/// `s.filter4(c)`
fn filter4(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanTurnSet()
    card::is_faceup(d, c) && card::is_can_turn_set(d, c)
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if e:GetLabel()==0 then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter1(chkc)
        if effect::get_label(d, a.e) == 0 {
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter1(d, chkc);
        }
        // else return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter3(chkc) end
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter3(d, chkc);
    }
    // local b1=Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,0,LOCATION_MZONE,1,nil)
    let b1 = duel::is_existing_target(d, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&filter2), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.filter3,tp,LOCATION_MZONE,0,1,nil)
    let b2 = duel::is_existing_target(d, Some(&filter3), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    // if b1 and b2 then
    let op: i64 = if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) as i64
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) as i64
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))+1
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) as i64 + 1
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, a.tp, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as i32);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWNDEFENSE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEDOWNDEFENSE as u64);
        // local g=Duel.SelectTarget(tp,s.filter3,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&filter3), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, a.tp, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as i32);
    }
    true
}
/// `s.filter3(c)`
fn filter3(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE)
    card::is_position(d, c, POS_FACEDOWN_DEFENSE)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
        let tc = tc.nil();
        if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
            return;
        }
        // if Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)==0 then return end
        if duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWNDEFENSE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEDOWNDEFENSE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&filter2), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.ChangePosition(g,POS_FACEUP_ATTACK)
            duel::change_position(d, g, POS_FACEUP_ATTACK as u8, None, None, None, false);
        }
    } else {
        // if not tc:IsRelateToEffect(e) or tc:IsPosition(POS_FACEUP_ATTACK) then return end
        let tc = tc.nil();
        if !card::is_relate_to_effect(d, tc, a.e) || card::is_position(d, tc, POS_FACEUP_ATTACK) {
            return;
        }
        // if Duel.ChangePosition(tc,POS_FACEUP_ATTACK)==0 or not tc:IsSetCard(SET_GHOSTRICK) then return end
        if duel::change_position(d, tc, POS_FACEUP_ATTACK as u8, None, None, None, false) == 0 || !card::is_set_card(d, tc, &[SET_GHOSTRICK as u16]) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter4,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&filter4), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
