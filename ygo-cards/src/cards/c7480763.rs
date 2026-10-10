//! アーティファクト-ダグザ
//! Artifact Dagda
//! scripted by Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 7480763;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTIFACT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,2,s.matfilter)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(matfilter)), None, None);
    // Card/effect on the field is activated, set an "Artifact" from hand/deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.setcon)
    effect::set_condition(d, e1, Some(condition(setcon)));
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon 1 "Artifact" monster from GY in defense position
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
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(g,lc,sumtype,tp)`: "Monsters with different names"
fn matfilter(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)
    group::check_different_property(d, g, &code_list)
}
fn code_list(d: &mut Duel, c: CardId) -> Vec<i64> {
    card::get_code(d, c).iter().map(|x| *x as i64).collect()
}
/// `s.setfilter(c)`: an "Artifact" monster that can be set as a spell
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARTIFACT) and c:IsMonster() and c:IsSSetable(true)
    card::is_set_card(d, c, &[SET_ARTIFACT as u16]) && card::is_monster(d, c) && card::is_s_setable(d, c, true, None)
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`: the card/effect (other than this card) was activated on the field
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return (loc&LOCATION_ONFIELD)~=0 and re:GetHandler()~=e:GetHandler() and re:GetHandler()~=e:GetHandler()
    if (loc & LOCATION_ONFIELD as i64) == 0 {
        return false;
    }
    let re = a.re.nil();
    let h = effect::get_handler(d, a.e).nil();
    effect::get_handler(d, re) != Some(h) && effect::get_handler(d, re) != Some(h)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`: activation legality
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`: set an "Artifact" from hand/deck, but destroy it later
fn setop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // Duel.SSet(tp,tc)
        duel::s_set(d, tp, tc, None, true);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::card(Some(tc)));
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // if Duel.IsPhase(PHASE_END) and Duel.IsTurnPlayer(1-tp) then
        if duel::is_phase(d, PHASE_END) && duel::is_turn_player(d, 1 - tp) {
            // e1:SetLabel(Duel.GetTurnCount())
            let turn = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[turn]);
            // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,2)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 2);
        } else {
            // e1:SetLabel(0)
            effect::set_label(d, e1, &[0]);
            // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // tc:CreateEffectRelation(e1)
        card::create_effect_relation(d, tc, e1);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`: check for flag
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card();
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetTurnCount()~=e:GetLabel() and tc:IsRelateToEffect(e)
    duel::is_turn_player(d, 1 - tp)
        && duel::get_turn_count(d, None) as i64 != effect::get_label(d, e)
        && card::is_relate_to_effect(d, tc.nil(), e)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`: destroy the set "Artifact" card
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card();
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, tc.nil(), REASON_EFFECT, None, None);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`: if this link summoned card is destroyed during opponent's turn
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsLinkSummoned() and Duel.GetTurnPlayer()~=tp
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_link_summoned(d, c) && duel::get_turn_player(d) != tp
}
/// `s.spfilter(c,e,tp)`: an "Artifact" monster
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ARTIFACT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_ARTIFACT as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`: activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`: special summon 1 "Artifact" monster from GY in defense position
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
