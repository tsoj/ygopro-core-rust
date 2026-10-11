//! Blockout Curtain
//! 遮攻カーテン
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 53167658;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace yours
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_value(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy replace opps
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetTarget(s.reptg2)
    effect::set_target(d, e3, Some(cost_target(reptg2)));
    // e3:SetValue(s.repval2)
    effect::set_value_fn(d, e3, effect_card_value(repval2));
    // e3:SetOperation(s.repop2)
    effect::set_operation(d, e3, Some(operation(repop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and not c:IsReason(REASON_REPLACE)
    //   and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED+STATUS_BATTLE_DESTROYED)
    //   and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::get_flag_effect(d, a.tp, ID) == 0
            && card::is_destructable(d, c, Some(a.e))
            && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,c,aux.Stringid(id,0)) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(aux::stringid(ID, 0))) {
        // local g=eg:Filter(s.repfilter,nil,tp)
        let eg = a.eg.clone().nil();
        let g = group::filter(d, &eg, &|d, x| repfilter(d, x, a.tp), NO_EX);
        // if #g==1 then e:SetLabelObject(g:GetFirst())
        if g.len() == 1 {
            let tc = group::get_first(d, &g).nil();
            effect::set_label_object(d, a.e, Value::Card(tc));
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
            // local cg=g:Select(tp,1,1,nil)
            let cg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // e:SetLabelObject(cg:GetFirst())
            let tc = group::get_first(d, &cg).nil();
            effect::set_label_object(d, a.e, Value::Card(tc));
        }
        // c:SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, c, STATUS_DESTROY_CONFIRMED, true);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repval(e,c)`: `return c==e:GetLabelObject()`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    (effect::get_label_object(d, e).as_card() == Some(c)) as i64
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, c, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(c,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, c, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.repfilter2(c,tp)`
fn repfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_controler(d, c, 1 - tp) && !card::is_reason(d, c, REASON_REPLACE) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.reptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetFlagEffect(tp,id+1)==0 and c:IsAbleToRemove() and eg:IsExists(s.repfilter2,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::get_flag_effect(d, a.tp, ID + 1) == 0
            && card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &eg, &|d, x| repfilter2(d, x, a.tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,c,aux.Stringid(id,1)) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(aux::stringid(ID, 1))) {
        // local g=eg:Filter(s.repfilter2,nil,tp)
        let eg = a.eg.clone().nil();
        let g = group::filter(d, &eg, &|d, x| repfilter2(d, x, a.tp), NO_EX);
        // if #g==1 then e:SetLabelObject(g:GetFirst())
        if g.len() == 1 {
            let tc = group::get_first(d, &g).nil();
            effect::set_label_object(d, a.e, Value::Card(tc));
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
            // local cg=g:Select(tp,1,1,nil)
            let cg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // e:SetLabelObject(cg:GetFirst())
            let tc = group::get_first(d, &cg).nil();
            effect::set_label_object(d, a.e, Value::Card(tc));
        }
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repval2(e,c)`: `return c==e:GetLabelObject()`
fn repval2(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    (effect::get_label_object(d, e).as_card() == Some(c)) as i64
}
/// `s.repop2(e,tp,eg,ep,ev,re,r,rp)`
fn repop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
