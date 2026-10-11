//! Magical Musket Mastermind Zakiel
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 30907810;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICAL_MUSKET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be tribute Summoned using 1 "Magical Musket" monster
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(Arc::new(otfilter)), None);
    // "Magical Musket" Spell/Traps can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_QP_ACT_IN_NTPHAND)
    effect::set_code(d, e2, EFFECT_QP_ACT_IN_NTPHAND);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MAGICAL_MUSKET))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e3, EFFECT_TRAP_ACT_IN_HAND);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Draw cards during the opponent's End Phase
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_DRAW);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e4, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // e5:SetOperation(s.regop)
    effect::set_operation(d, e5, Some(operation(regop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e6, EVENT_CHAIN_NEGATED);
    // e6:SetOperation(s.regop2)
    effect::set_operation(d, e6, Some(operation(regop2)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    // e7:SetCode(EVENT_PHASE_START|PHASE_DRAW)
    effect::set_code(d, e7, EVENT_PHASE_START | PHASE_DRAW);
    // e7:SetOperation(s.clearop)
    effect::set_operation(d, e7, Some(operation(clearop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // local e8=e6:Clone()
    let e8 = effect::clone(d, e6);
    // e8:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e8:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e8, EVENT_LEAVE_FIELD_P);
    // e8:SetOperation(s.clearop)
    effect::set_operation(d, e8, Some(operation(clearop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
// s.otfilter(c,tp)
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MAGICAL_MUSKET) and (c:IsControler(tp) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
// s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // if re:GetHandler():IsSetCard(SET_MAGICAL_MUSKET) and rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_set_card(d, rh, &[SET_MAGICAL_MUSKET as u16]) && a.rp == a.tp && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        // local val=e:GetLabelObject():GetLabel()
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        let val = effect::get_label(d, lo);
        // e:GetLabelObject():SetLabel(val+1)
        effect::set_label(d, lo, &[val + 1]);
    }
}
// s.regop2(e,tp,eg,ep,ev,re,r,rp)
fn regop2(d: &mut Duel, a: &EffArgs) {
    // if re:GetHandler():IsSetCard(SET_MAGICAL_MUSKET) and rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_set_card(d, rh, &[SET_MAGICAL_MUSKET as u16]) && a.rp == a.tp && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        // local val=e:GetLabelObject():GetLabel()
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        let mut val = effect::get_label(d, lo);
        // if val==0 then val=1 end
        if val == 0 {
            val = 1;
        }
        // e:GetLabelObject():SetLabel(val-1)
        effect::set_label(d, lo, &[val - 1]);
    }
}
// s.clearop(e,tp,eg,ep,ev,re,r,rp)
fn clearop(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(0)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[0]);
}
// s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=e:GetLabel()
    let dr = effect::get_label(d, a.e);
    // if chk==0 then return d>0 and Duel.IsPlayerCanDraw(tp,d) end
    if a.chk == Some(0) {
        return dr > 0 && duel::is_player_can_draw(d, a.tp, dr as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,d)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, dr as i32);
    true
}
// s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    // local d=e:GetLabel()
    let dr = effect::get_label(d, a.e);
    // if d>0 then
    if dr > 0 {
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, p, dr as u32, REASON_EFFECT);
    }
}
