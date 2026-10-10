//! Seven Cities of the Golden Land
use super::prelude::*;

pub const ID: u32 = 95034141;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local params = {nil,aux.FilterBoolFunction(Card.IsRace,RACE_ZOMBIE)}
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, summon_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op = fusion::summon_eff_op(d, summon_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.stcon)
    effect::set_condition(d, e3, Some(condition(stcon)));
    // e3:SetTarget(s.sttg)
    effect::set_target(d, e3, Some(cost_target(sttg)));
    // e3:SetOperation(s.stop)
    effect::set_operation(d, e3, Some(operation(stop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// {nil, aux.FilterBoolFunction(Card.IsRace,RACE_ZOMBIE)} as the named parameters of Fusion.SummonEffTG / OP
fn summon_params() -> fusion::SummonEff {
    fusion::SummonEff {
        matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_race(d, c, RACE_ZOMBIE))),
        ..Default::default()
    }
}
// function s.stfilter(c,tp)
fn stfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE)
    card::is_race(d, c, RACE_ZOMBIE)
}
// function s.stcon(e,tp,eg,ep,ev,re,r,rp)
fn stcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.stfilter,1,nil,tp)
    //     and (re:IsHasType(EFFECT_TYPE_ACTIONS) or re:IsHasType(EFFECT_TYPE_CONTINUOUS))
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &stfilter, 1, NO_EX) && {
        let re = a.re.nil();
        effect::is_has_type(d, re, EFFECT_TYPE_ACTIONS) || effect::is_has_type(d, re, EFFECT_TYPE_CONTINUOUS)
    }
}
// function s.filter(c,rc)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()~=5
    card::is_facedown(d, c) && card::get_sequence(d, c) != 5
}
// function s.sttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWN)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEDOWN as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX);
    true
}
// function s.stop(e,tp,eg,ep,ev,re,r,rp)
fn stop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
                // c:SetCardTarget(tc)
                card::set_card_target(d, c, tc);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_CANNOT_TRIGGER)
                effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
