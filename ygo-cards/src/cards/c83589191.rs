//! Life Hack
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 83589191;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted monster's ATK becomes your opponent's current LP
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    effect::set_label(d, e1, &[1]);
    card::register_effect(d, c, e1, false);
    // Targeted monster's ATK becomes your current LP
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_label(d, e2, &[2]);
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,lp)`
fn tgfilter(d: &mut Duel, c: CardId, lp: i32) -> bool {
    // return c:IsFaceup() and not c:IsAttack(lp)
    card::is_faceup(d, c) && !card::is_attack(d, c, &[lp as u32])
}
/// `s.atktg(e,...,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local p=e:GetLabel()==1 and 1-tp or tp
    let p = if effect::get_label(d, a.e) == 1 { 1 - a.tp } else { a.tp };
    // local lp=Duel.GetLP(p)
    let lp = duel::get_lp(d, p);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,lp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, lp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,lp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, lp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,lp)
    let tc = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, lp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, tc, 1, a.tp, 0);
    true
}
/// `s.atkop(e,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local p=e:GetLabel()==1 and 1-tp or tp
        let p = if effect::get_label(d, a.e) == 1 { 1 - a.tp } else { a.tp };
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(Duel.GetLP(p))
        let lp = duel::get_lp(d, p);
        effect::set_value(d, e1, lp as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_DAMAGE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(function(e,re,val,r,rp,rc) return val//2 end)
    effect::set_value_fn(d, e2, lua_fn(halve));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `function(e,re,val,r,rp,rc) return val//2 end`
fn halve(_d: &mut Duel, a: Args) -> i64 {
    a.int(2).div_euclid(2)
}
