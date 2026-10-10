//! Forbidden Crown
use super::prelude::*;

pub const ID: u32 = 98829635;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply these effects to 1 face-up monster on the field, until the end of this turn
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(aux.NOT(Card.HasFlagEffect),id)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::has_flag_effect(d, c, ID, None)
}
/// `function(te) return not te:IsMonsterEffect() end`
fn chain_limit(d: &mut Duel, a: Args) -> bool {
    let te = a.e(0);
    !effect::is_monster_effect(d, te)
}
/// `function(e,re) return re:IsActivated() and e:GetHandler()~=re:GetOwner() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    effect::is_activated(d, re) && effect::get_handler(d, e) != effect::get_owner(d, re)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(aux.NOT(Card.HasFlagEffect),id),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetChainLimit(function(te) return not te:IsMonsterEffect() end)
    duel::set_chain_limit(d, lua_fn(chain_limit));
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(aux.NOT(Card.HasFlagEffect),id),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // sc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
        card::register_flag_effect(d, sc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
        // sc:NegateEffects(c,RESETS_STANDARD_PHASE_END)
        card::negate_effects(d, sc, c, Some(RESETS_STANDARD_PHASE_END), false, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // local e2=e1:Clone()
        // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        // e2:SetValue(1)
        // sc:RegisterEffect(e2)
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e2, 1);
        card::register_effect(d, sc, e2, false);
        // local e3=e2:Clone()
        // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        // sc:RegisterEffect(e3)
        let e3 = effect::clone(d, e2);
        effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
        card::register_effect(d, sc, e3, false);
        // local e4=e1:Clone()
        // e4:SetCode(EFFECT_IMMUNE_EFFECT)
        // e4:SetValue(function(e,re) return re:IsActivated() and e:GetHandler()~=re:GetOwner() end)
        // sc:RegisterEffect(e4)
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e4, lua_fn(immune_value));
        card::register_effect(d, sc, e4, false);
        // local e5=e2:Clone()
        // e5:SetCode(EFFECT_UNRELEASABLE_SUM)
        // sc:RegisterEffect(e5)
        let e5 = effect::clone(d, e2);
        effect::set_code(d, e5, EFFECT_UNRELEASABLE_SUM);
        card::register_effect(d, sc, e5, false);
        // local e6=e2:Clone()
        // e6:SetCode(EFFECT_UNRELEASABLE_NONSUM)
        // sc:RegisterEffect(e6)
        let e6 = effect::clone(d, e2);
        effect::set_code(d, e6, EFFECT_UNRELEASABLE_NONSUM);
        card::register_effect(d, sc, e6, false);
        // local e7=e1:Clone()
        // e7:SetCode(EFFECT_CANNOT_BE_MATERIAL)
        // e7:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
        // sc:RegisterEffect(e7)
        let e7 = effect::clone(d, e1);
        effect::set_code(d, e7, EFFECT_CANNOT_BE_MATERIAL);
        effect::set_value_fn(d, e7, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
        card::register_effect(d, sc, e7, false);
    }
}
