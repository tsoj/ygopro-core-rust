//! Speedroid Colonel Clackers
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 65155517;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[65155517], listed_series: &[SET_SPEEDROID, SET_CLEAR_WING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Place 1 "Speedroid" Pendulum Monster from your Deck in your Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.placecost)
    effect::set_cost(d, e1, Some(cost_target(placecost)));
    // e1:SetTarget(s.placetg)
    effect::set_target(d, e1, Some(cost_target(placetg)));
    // e1:SetOperation(s.placeop)
    effect::set_operation(d, e1, Some(operation(placeop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Speedroid Clackernel" from your hand, Deck, or GY as a Tuner
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(condition(cond1)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish this card and increase the ATK of a "Clear Wing" monster by 700
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e3, LOCATION_EXTRA);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.placecost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn placecost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.placefilter(c)`
fn placefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
/// `s.placetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn placetg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) and Duel.IsExistingMatchingCard(s.placefilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp) && duel::is_existing_matching_card(d, Some(&placefilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.placeop(e,tp,eg,ep,ev,re,r,rp)`
fn placeop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler() (for the Effect.CreateEffect owner below)
    let c = effect::get_handler(d, a.e).nil();
    // You cannot Special Summon for the rest of this turn, except WIND monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsAttribute(ATTRIBUTE_WIND) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_attribute(d, c, ATTRIBUTE_WIND))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.placefilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&placefilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `c:AssumeProperty(ASSUME_TYPE,c:GetType()|TYPE_TUNER)`
fn assume_tuner(d: &mut Duel, c: CardId) {
    let t = card::get_type(d, c);
    card::assume_property(d, c, ASSUME_TYPE, (t | TYPE_TUNER) as u64);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not c:IsCode(id) then return end
    if !card::is_code(d, c, &[ID]) {
        return false;
    }
    // c:AssumeProperty(ASSUME_TYPE,c:GetType()|TYPE_TUNER)
    assume_tuner(d, c);
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // tc:AssumeProperty(ASSUME_TYPE,tc:GetType()|TYPE_TUNER)
    assume_tuner(d, tc);
    // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // Treat it as a Tuner
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.spcon` of e2: `r&(REASON_BATTLE|REASON_EFFECT)>0 and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    if (a.r & (REASON_BATTLE | REASON_EFFECT)) == 0 {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    // if chk==0 then return c:IsAbleToRemove() and bc and bc:IsSetCard(SET_CLEAR_WING) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None) && bc.is_some_and(|bc| card::is_set_card(d, bc, &[SET_CLEAR_WING as u16]));
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,bc,1,tp,700)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, bc.map(CardSet::from_card), 1, a.tp, 700);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    // if c:IsRelateToEffect(e) and Duel.Remove(c,POS_FACEUP,REASON_EFFECT)>0
    //     and bc and bc:IsRelateToBattle() then
    if card::is_relate_to_effect(d, c, a.e) && duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        if let Some(bc) = bc {
            if card::is_relate_to_battle(d, bc) {
                // It gains 700 ATK
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(700)
                effect::set_value(d, e1, 700);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // bc:RegisterEffect(e1)
                card::register_effect(d, bc, e1, false);
            }
        }
    }
}
