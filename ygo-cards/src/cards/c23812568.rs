//! Additional Mirror Level 7
use super::prelude::*;

pub const ID: u32 = 23812568;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 monsters with the same name from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.confilter(c,tp)`
fn confilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevel(7) and c:IsFaceup() and c:IsControler(tp)
    card::is_level(d, c, &[7]) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.confilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| confilter(d, c, a.tp), 1, NO_EX)
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[ID]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_DECK,0,2,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 2, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_DECK,0,2,2,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tgfilter(c,e,tp,eg)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, eg: &CardSet) -> bool {
    // return eg:IsContains(c) and c:IsLevel(7) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,2,nil,e,tp,c:GetCode())
    if !(group::is_contains(eg, c) && card::is_level(d, c, &[7]) && card::is_faceup(d, c)) {
        return false;
    }
    // (c:GetCode() is the last argument: its first code is the one the filter gets)
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 2, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // (the Lua tests `chkc`, a global that is nil here: no target-check branch)
    let eg = a.eg.clone().nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and ft>=2
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp,eg) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && ft >= 2
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.e, a.tp, &eg)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,eg)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.e, a.tp, &eg)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, a.tp, 0);
    true
}
/// `Card.GetBaseAttack` as the value of `Duel.GetOperatedGroup():GetSum`
fn base_attack_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_base_attack(d, c) as i64
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        // Can only Special Summon from the Extra Deck once
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
        // e1:SetCondition(s.spcon)
        effect::set_condition(d, e1, Some(condition(spcon)));
        // e1:SetOperation(s.spop)
        effect::set_operation(d, e1, Some(operation(spop)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Special Summon 2 monsters with the same name and take damage equal to the combined ATK
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,2,2,nil,e,tp,tc:GetCode())
    let code = card::get_code(d, tc)[0];
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, code)), a.tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local sum=Duel.GetOperatedGroup():GetSum(Card.GetBaseAttack)
        let operated = duel::get_operated_group(d);
        let sum = group::get_sum(d, &operated, &base_attack_value);
        // if sum>0 then
        if sum > 0 {
            // Duel.Damage(tp,sum,REASON_EFFECT)
            duel::damage(d, a.tp, sum, REASON_EFFECT, false, None);
        }
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_summon_player(d, c, tp) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `function(_,c) return c:IsLocation(LOCATION_EXTRA) end` (the target of `spop`'s effect)
fn extra_target(d: &mut Duel, a: Args) -> bool {
    card::is_location(d, a.c(1), LOCATION_EXTRA)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(lua_fn(extra_target)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
