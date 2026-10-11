//! Eternal Sunshine
//! エターナル・サンシャイン
use super::prelude::*;

pub const ID: u32 = 28903523;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_FAIRY_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e0, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e0, TIMING_DAMAGE_STEP as u32, None);
    // e0:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e0, Some(lua_fn(|d, _a: Args| aux::stat_change_damage_step_condition(d))));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Monsters you control gain 500 DEF for each "Ancient Fairy Dragon", and monster that mentions it, that you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetValue(s.defval)
    effect::set_value_fn(d, e1, effect_card_value(defval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Halve the current ATK/DEF and negate the effects of 1 opponent's monster until the end of that turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP as u32, Some((TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.afdfilter(c)`
fn afdfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_ANCIENT_FAIRY_DRAGON) or (c:ListsCode(CARD_ANCIENT_FAIRY_DRAGON) and c:IsMonster())) and c:IsFaceup()
    (card::is_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON]) || (card::lists_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON]) && card::is_monster(d, c)))
        && card::is_faceup(d, c)
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.afdfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,nil)*500
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&afdfilter), p, LOCATION_ONFIELD, 0, NO_EX) as i64 * 500
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.GetFlagEffect(tp,id)<Duel.GetMatchingGroupCount(s.afdfilter,tp,LOCATION_ONFIELD,0,nil)
    //     and aux.StatChangeDamageStepCondition()
    duel::get_flag_effect(d, tp, ID) < duel::get_matching_group_count(d, Some(&afdfilter), tp, LOCATION_ONFIELD, 0, NO_EX)
        && aux::stat_change_damage_step_condition(d)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Its current ATK/DEF become halved until the end of this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(math.ceil(tc:GetAttack()/2))
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, ((atk as f64) / 2.0).ceil() as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(math.ceil(tc:GetDefense()/2))
        let def = card::get_defense(d, tc);
        effect::set_value(d, e2, ((def as f64) / 2.0).ceil() as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Its effects are negated until the end of this turn
        // tc:NegateEffects(c,RESETS_STANDARD_PHASE_END)
        card::negate_effects(d, tc, c, Some(RESETS_STANDARD_PHASE_END), false, None);
    }
}
