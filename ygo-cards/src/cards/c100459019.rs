//! Attribute Rejection
use super::prelude::*;

pub const ID: u32 = 100459019;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_ATKCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(
        d,
        e1,
        TIMING_DAMAGE_STEP,
        Some(TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.togyfilter(c)`
fn togyfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsAttribute(ATTRIBUTE_DARK) or c:IsRace(RACE_ZOMBIE)) and (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE))
    //     and c:IsAbleToGrave()
    (card::is_attribute(d, c, ATTRIBUTE_DARK) || card::is_race(d, c, RACE_ZOMBIE))
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE))
        && card::is_able_to_grave(d, c, None)
}
/// `function(effect,event_player,triggering_player) return triggering_player==event_player end`
fn chain_limit(_d: &mut Duel, a: Args) -> bool {
    a.p(2) == a.p(1)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsSummonLocation(LOCATION_EXTRA) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_faceup(d, chkc)
            && card::is_summon_location(d, chkc, LOCATION_EXTRA);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSummonLocation,LOCATION_EXTRA),tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(s.togyfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, x| card::is_faceup(d, x) && card::is_summon_location(d, x, LOCATION_EXTRA)),
            tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        ) && duel::is_existing_matching_card(d, Some(&togyfilter), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSummonLocation,LOCATION_EXTRA),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(
        d,
        tp,
        Some(&|d, x| card::is_faceup(d, x) && card::is_summon_location(d, x, LOCATION_EXTRA)),
        tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g.clone()), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, tp, 0);
    // if Duel.GetCurrentChain()>=3 and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if duel::get_current_chain(d, false) >= 3 && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(function(effect,event_player,triggering_player) return triggering_player==event_player end)
        duel::set_chain_limit(d, lua_fn(chain_limit));
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.togyfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&togyfilter), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE)
    //     and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    let Some(sc) = sc else {
        return;
    };
    if duel::send_to_grave(d, CardSet::from_card(sc), REASON_EFFECT, None, None) > 0
        && card::is_location(d, sc, LOCATION_GRAVE)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && card::is_faceup(d, tc.nil())
    {
        let tc = tc.nil();
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local prev_atk=tc:GetAttack()
        let prev_atk = card::get_attack(d, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-sc:GetBaseAttack())
        let base_atk = card::get_base_attack(d, sc);
        effect::set_value(d, e1, -(base_atk as i64));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if prev_atk>tc:GetAttack() and tc:IsAttribute(ATTRIBUTE_LIGHT) and tc:IsNegatableMonster() then
        let atk_now = card::get_attack(d, tc);
        if prev_atk > atk_now && card::is_attribute(d, tc, ATTRIBUTE_LIGHT) && card::is_negatable_monster(d, tc) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // tc:NegateEffects(c)
            card::negate_effects(d, tc, c, None, false, None);
        }
    }
}
