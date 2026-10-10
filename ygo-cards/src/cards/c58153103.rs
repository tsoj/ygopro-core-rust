//! Armed Dragon Thunder LV10
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 58153103;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[59464593],
    listed_series: &[SET_ARMED_DRAGON],
    lv: Some((10, SET_ARMED_DRAGON_THUNDER)),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Register if this card is Special Summoned by the effect of an "Armed Dragon" monster
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.regcon)
    effect::set_condition(d, e0, Some(condition(regcon)));
    // e0:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TEMP_REMOVE,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2)) end)
    effect::set_operation(d, e0, Some(lua_fn(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // ICHI - This card's name becomes "Armed Dragon LV10"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon(1))
    effect::set_condition(d, e1, Some(condition(|d, a| atkcon(d, a.e, 1))));
    // e1:SetValue(59464593)
    effect::set_value(d, e1, 59464593);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // JUU - Control of this card cannot switch
    // local e10=Effect.CreateEffect(c)
    let e10 = effect::create_effect(d, c);
    // e10:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e10, EFFECT_TYPE_SINGLE);
    // e10:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e10, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e10:SetCode(EFFECT_CANNOT_CHANGE_CONTROL)
    effect::set_code(d, e10, EFFECT_CANNOT_CHANGE_CONTROL);
    // e10:SetRange(LOCATION_MZONE)
    effect::set_range(d, e10, LOCATION_MZONE);
    // e10:SetCondition(s.atkcon(10))
    effect::set_condition(d, e10, Some(condition(|d, a| atkcon(d, a.e, 10))));
    // c:RegisterEffect(e10)
    card::register_effect(d, c, e10, false);
    // HYAKU - Cannot be destroyed by battle
    // local e100=Effect.CreateEffect(c)
    let e100 = effect::create_effect(d, c);
    // e100:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e100, EFFECT_TYPE_SINGLE);
    // e100:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e100, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e100:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e100, EFFECT_INDESTRUCTABLE_BATTLE);
    // e100:SetRange(LOCATION_MZONE)
    effect::set_range(d, e100, LOCATION_MZONE);
    // e100:SetCondition(s.atkcon(100))
    effect::set_condition(d, e100, Some(condition(|d, a| atkcon(d, a.e, 100))));
    // e100:SetValue(1)
    effect::set_value(d, e100, 1);
    // c:RegisterEffect(e100)
    card::register_effect(d, c, e100, false);
    // SEN - Destroy 1 other card on the field and make this card gain 1000 ATK
    // local e1000=Effect.CreateEffect(c)
    let e1000 = effect::create_effect(d, c);
    // e1000:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1000, aux::stringid(ID, 0));
    // e1000:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1000, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e1000:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1000, EFFECT_TYPE_QUICK_O);
    // e1000:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1000, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1000:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1000, EVENT_FREE_CHAIN);
    // e1000:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1000, LOCATION_MZONE);
    // e1000:SetCountLimit(1)
    effect::set_count_limit(d, e1000, 1, 0, 0, 0);
    // e1000:SetHintTiming(0,TIMING_DAMAGE_STEP|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1000, 0, Some(TIMING_DAMAGE_STEP | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1000:SetCondition(s.descon)
    effect::set_condition(d, e1000, Some(condition(descon)));
    // e1000:SetCost(s.descost)
    effect::set_cost(d, e1000, Some(cost_target(descost)));
    // e1000:SetTarget(s.destg)
    effect::set_target(d, e1000, Some(cost_target(destg)));
    // e1000:SetOperation(s.desop)
    effect::set_operation(d, e1000, Some(operation(desop)));
    // c:RegisterEffect(e1000)
    card::register_effect(d, c, e1000, false);
    // MANJOUME THUNDER - Destroy all other cards on the field
    // local e10000=Effect.CreateEffect(c)
    let e10000 = effect::create_effect(d, c);
    // e10000:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e10000, aux::stringid(ID, 1));
    // e10000:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e10000, CATEGORY_DESTROY);
    // e10000:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e10000, EFFECT_TYPE_IGNITION);
    // e10000:SetRange(LOCATION_MZONE)
    effect::set_range(d, e10000, LOCATION_MZONE);
    // e10000:SetCountLimit(1)
    effect::set_count_limit(d, e10000, 1, 0, 0, 0);
    // e10000:SetCondition(s.atkcon(10000))
    effect::set_condition(d, e10000, Some(condition(|d, a| atkcon(d, a.e, 10000))));
    // e10000:SetTarget(s.desalltg)
    effect::set_target(d, e10000, Some(cost_target(desalltg)));
    // e10000:SetOperation(s.desallop)
    effect::set_operation(d, e10000, Some(operation(desallop)));
    // c:RegisterEffect(e10000)
    card::register_effect(d, c, e10000, false);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsCardType(TYPE_MONSTER) and re:IsCardSetcode(SET_ARMED_DRAGON)
    match a.re {
        Some(re) => effect::is_card_type(d, re, TYPE_MONSTER) && effect::is_card_setcode(d, re, &[SET_ARMED_DRAGON as u16]),
        None => false,
    }
}
/// `function(e) e:GetHandler():RegisterFlagEffect(...) end`
fn regop(d: &mut Duel, a: Args) {
    let h = effect::get_handler(d, a.e(0)).nil();
    // RESET_EVENT|RESETS_STANDARD&~RESET_TEMP_REMOVE
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TEMP_REMOVE), EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
}
/// `s.atkcon(atk)`: `function(e,tp,eg,ep,ev,re,r,rp) local c=e:GetHandler() return c:HasFlagEffect(id) and c:IsAttackAbove(atk) end`
fn atkcon(d: &mut Duel, e: EffectId, atk: i64) -> bool {
    let c = effect::get_handler(d, e).nil();
    card::has_flag_effect(d, c, ID, None) && card::is_attack_above(d, c, atk as i32)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and s.atkcon(1000)(e,tp,eg,ep,ev,re,r,rp)
    //     and not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated())
    duel::is_turn_player(d, 1 - a.tp) && atkcon(d, a.e, 1000) && !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,1,1,REASON_COST)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), 1, 1, REASON_COST, NO_EX);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then c:UpdateAttack(1000) end
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        card::update_attack(d, c, 1000, None, None, None);
    }
}
/// `s.desalltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desalltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desallop(e,tp,eg,ep,ev,re,r,rp)`
fn desallop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc: Exclude = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,exc)
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
