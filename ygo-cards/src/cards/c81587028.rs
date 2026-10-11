//! Box of Friends
use super::prelude::*;

pub const ID: u32 = 81587028;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 2 normal monsters, with 0 ATK or DEF and different names, from deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_DESTROY)
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_DESTROY)
}
// function s.filter1(c,e,tp)
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and (c:GetAttack()==0 or c:IsDefenseBelow(0)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_type(d, c, TYPE_NORMAL)
        && (card::get_attack(d, c) == 0 || card::is_defense_below(d, c, 0))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return false end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return false;
        }
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return false end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>=2
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)>=2 then
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2 {
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(e),
            tp,
            Some(2),
            Some(2),
            Some(&aux::dncheck()),
            1,
            tp,
            Some(HINTMSG_SPSUMMON as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // local tc1=sg:GetFirst()
        let tc1 = group::get_first(d, &sg).nil();
        // local tc2=sg:GetNext()
        let order = group::iter(d, &sg);
        let tc2 = order.get(1).copied().nil();
        // Duel.SpecialSummonStep(tc1,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, tc1, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // Duel.SpecialSummonStep(tc2,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, tc2, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // tc1:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, tc1, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // tc2:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, tc2, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // Cannot be used as synchro material
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3310)
        effect::set_description(d, e1, 3310);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
        effect::set_code(d, e1, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc1:RegisterEffect(e1)
        card::register_effect(d, tc1, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // tc2:RegisterEffect(e2)
        card::register_effect(d, tc2, e2, false);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg:KeepAlive()
        // Destroyed during next end phase
        // local de=Effect.CreateEffect(e:GetHandler())
        let de = effect::create_effect(d, h);
        // de:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, de, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // de:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, de, EVENT_PHASE + PHASE_END);
        // de:SetCountLimit(1)
        effect::set_count_limit(d, de, 1, 0, 0, 0);
        // de:SetLabelObject(sg)
        effect::set_label_object(d, de, Value::Group(sg.clone()));
        // de:SetCondition(s.descon)
        effect::set_condition(d, de, Some(condition(descon)));
        // de:SetOperation(s.desop)
        effect::set_operation(d, de, Some(operation(desop)));
        // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_END) then
        if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_END) {
            // de:SetLabel(Duel.GetTurnCount())
            let turn_count = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, de, &[turn_count]);
            // de:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
            effect::set_reset(d, de, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
        } else {
            // de:SetLabel(0)
            effect::set_label(d, de, &[0]);
            // de:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN)
            effect::set_reset(d, de, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
        }
        // Duel.RegisterEffect(de,tp)
        duel::register_effect(d, de, tp);
    }
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    // if not g:IsExists(s.desfilter,1,nil) then
    if !group::is_exists(d, &g, &desfilter, 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, e);
        // return false
        return false;
    }
    // return Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel()
    duel::is_turn_player(d, a.tp) && duel::get_turn_count(d, None) as i64 != effect::get_label(d, e)
}
// function s.desfilter(c)
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)>0
    card::get_flag_effect(d, c, ID) > 0
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    // local tg=g:Filter(s.desfilter,nil)
    let tg = group::filter(d, &g, &desfilter, NO_EX);
    // g:DeleteGroup()
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
