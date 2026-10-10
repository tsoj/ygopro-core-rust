//! The Weather Painter Moonbow
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 95515518;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[54178659], listed_series: &[SET_THE_WEATHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_THE_WEATHER),3,3)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_THE_WEATHER as u16], sc, st, Some(tp)))),
        3,
        Some(3),
        None,
        None,
        None,
    );
    // Special Summon 1 of your banished "The Weather" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(condition(spcon1)));
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(cost_target(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(operation(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "The Weather Painter Rainbow" from your Extra Deck
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
    // e2:SetCondition(s.spcon2)
    effect::set_condition(d, e2, Some(condition(spcon2)));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop2)
    effect::set_operation(d, e2, Some(operation(spop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Grant banishing effect
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e4, CATEGORY_REMOVE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCost(s.rmcost)
    effect::set_cost(d, e4, Some(cost_target(rmcost)));
    // e4:SetTarget(s.rmtg)
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    // e4:SetOperation(s.rmop)
    effect::set_operation(d, e4, Some(operation(rmop)));
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e5, LOCATION_MZONE, 0);
    // e5:SetTarget(s.eftg)
    effect::set_target(d, e5, Some(effect_card_fn(eftg)));
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.spcon1(e,tp,eg,ep,ev,re,r,rp)`
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_THE_WEATHER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_THE_WEATHER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsLinkSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_link_summoned(d, c)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(54178659) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[54178659])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.eftg(e,c)`
fn eftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    // return c:IsType(TYPE_EFFECT) and c:IsSetCard(SET_THE_WEATHER) and lg:IsContains(c)
    card::is_type(d, c, TYPE_EFFECT) && card::is_set_card(d, c, &[SET_THE_WEATHER as u16]) && group::is_contains(&lg, c)
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` called from this card's cost
fn bfgcost(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    aux::bfgcost()
        .call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(chk),
            ],
        )
        .truthy()
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk) end
    if a.chk == Some(0) {
        return bfgcost(d, a, 0);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)
    bfgcost(d, a, a.chk.unwrap_or(1));
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, CardSet::from_card(tc), 0, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0 {
        // local ct=Duel.GetCurrentPhase()<=PHASE_STANDBY and 2 or 1
        let ct: u32 = if duel::get_current_phase(d) <= PHASE_STANDBY { 2 } else { 1 };
        // tc:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TEMP_REMOVE)|RESET_PHASE|PHASE_STANDBY,0,ct)
        card::register_flag_effect(
            d,
            tc,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !RESET_TEMP_REMOVE) | RESET_PHASE | PHASE_STANDBY,
            0,
            ct as u16,
            0,
            0,
        );
        // Banish it until the Standby Phase of the next turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY,ct)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, ct as u16);
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e1, &[turn]);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // tc:SetReasonEffect(e1)
        card::set_reason_effect(d, tc, e1);
    }
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()>e:GetLabel() and e:GetLabelObject():GetFlagEffect(id)>0
    let turn = duel::get_turn_count(d, None) as i64;
    let label = effect::get_label(d, a.e);
    let card_obj = effect::get_label_object(d, a.e).as_card().nil();
    turn > label && card::get_flag_effect(d, card_obj, ID) > 0
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ZONE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_ZONE as u64);
    // Duel.ReturnToField(e:GetLabelObject())
    let card_obj = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, card_obj, None, None);
}
