//! Diffusion Wave-Motion
use super::prelude::*;

pub const ID: u32 = 87880531;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_SPELLCASTER) and c:IsLevelAbove(7)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER as u64) && card::is_level_above(d, c, 7)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsControler(tp) and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_controler(d, tc, tp) && card::is_relate_to_effect(d, tc, e) {
        let h = effect::get_handler(d, e).nil();
        // local ae=Effect.CreateEffect(e:GetHandler())
        let ae = effect::create_effect(d, h);
        // ae:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ae, EFFECT_TYPE_FIELD);
        // ae:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, ae, EFFECT_CANNOT_ATTACK);
        // ae:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, ae, LOCATION_MZONE, 0);
        // ae:SetTarget(s.ftarget)
        effect::set_target(d, ae, Some(effect_card_fn(ftarget)));
        // ae:SetLabel(tc:GetFieldID())
        let fid = card::get_field_id(d, tc) as i64;
        effect::set_label(d, ae, &[fid]);
        // ae:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, ae, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(ae,tp)
        duel::register_effect(d, ae, tp);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_MUST_ATTACK)
        effect::set_code(d, e1, EFFECT_MUST_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, h);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_ATTACK_ALL)
        effect::set_code(d, e3, EFFECT_ATTACK_ALL);
        // e3:SetValue(1)
        effect::set_value(d, e3, 1);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // local e4=Effect.CreateEffect(e:GetHandler())
        let e4 = effect::create_effect(d, h);
        // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e4:SetCode(EVENT_BATTLED)
        effect::set_code(d, e4, EVENT_BATTLED);
        // e4:SetOperation(s.disop)
        effect::set_operation(d, e4, Some(operation(disop)));
        // e4:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
    }
}
/// `s.ftarget(e,c)`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // if not bc or not bc:IsStatus(STATUS_BATTLE_DESTROYED) then return end
    let Some(bc) = card::get_battle_target(d, c) else { return };
    if !card::is_status(d, bc, STATUS_BATTLE_DESTROYED) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetOwner())
    let owner = effect::get_owner(d, a.e).nil();
    let e1 = effect::create_effect(d, owner);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TOGRAVE))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TOGRAVE)), 1);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(lua_fn(con)));
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsMonster()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_monster(d, h)
}
