//! Ties of the Brethren
use super::prelude::*;

pub const ID: u32 = 40450317;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 monster from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,2000) and Duel.GetActivityCount(tp,ACTIVITY_BATTLE_PHASE)==0 end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 2000) && duel::get_activity_count(d, a.tp, &[ACTIVITY_BATTLE_PHASE])[0] == 0;
    }
    // Duel.PayLPCost(tp,2000)
    duel::pay_lp_cost(d, a.tp, 2000);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not c:IsFaceup() or not c:IsLevelBelow(4) then return false end
    if !card::is_faceup(d, c) || !card::is_level_below(d, c, 4) {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_DECK,0,nil,e,tp,c)
    let g = duel::get_matching_group(d, Some(&|d, x| filter2(d, x, e, tp, Some(c))), tp, LOCATION_DECK, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>1
    group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) > 1
}
/// s.filter2(c,e,tp,tc)
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: Option<CardId>) -> bool {
    let tc = tc.nil();
    // return c:GetLevel()==tc:GetLevel() and c:IsRace(tc:GetRace()) and c:IsAttribute(tc:GetAttribute())
    //     and not c:IsCode(tc:GetCode()) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    if card::get_level(d, c) != card::get_level(d, tc) {
        return false;
    }
    let race = card::get_race(d, tc);
    if !card::is_race(d, c, race) {
        return false;
    }
    let attr = card::get_attribute(d, tc);
    if !card::is_attribute(d, c, attr) {
        return false;
    }
    let codes = card::get_code(d, tc);
    if card::is_code(d, c, &codes) {
        return false;
    }
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_target(d, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, a.tp, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local g=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_DECK,0,nil,e,tp,tc)
    let g = duel::get_matching_group(d, Some(&|d, x| filter2(d, x, a.e, a.tp, tc)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and tc:IsRelateToEffect(e) and tc:IsFaceup() and #g>=2 then
    let affected = duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some();
    if !affected && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil()) && g.len() >= 2 {
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
        let dn = aux::dncheck();
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&dn), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
        // if #sg==2 then
        if sg.len() == 2 {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
