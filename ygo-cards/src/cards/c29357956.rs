//! Gladiator Beast Nerokius
use super::prelude::*;

pub const ID: u32 = 29357956;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR], material_setcode: &[SET_GLADIATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GLADIATOR),3)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GLADIATOR as u16], Some(fc), st, Some(tp)))), 3)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // battle indestructable
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // actlimit
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // e4:SetCondition(s.actcon)
    effect::set_condition(d, e4, Some(condition(actcon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // special summon
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCondition(s.spcon)
    effect::set_condition(d, e6, Some(condition(spcon)));
    // e6:SetCost(s.spcost)
    effect::set_cost(d, e6, Some(cost_target(spcost)));
    // e6:SetTarget(s.sptg)
    effect::set_target(d, e6, Some(cost_target(sptg)));
    // e6:SetOperation(s.spop)
    effect::set_operation(d, e6, Some(operation(spop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.contactfil(tp)
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(function(c) return c:IsMonster() and c:IsAbleToDeckOrExtraAsCost() end,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck_or_extra_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
// function s.contactop(g,tp)
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetLocation()~=LOCATION_EXTRA
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_location(d, h) != LOCATION_EXTRA
}
// function s.actcon(e)
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler() or Duel.GetAttackTarget()==e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(c) || duel::get_attack_target(d) == Some(c)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let c = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, c) > 0
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToExtraAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_extra_as_cost(d, c);
    }
    // Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_COST)
    duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKTOP as i32, REASON_COST, None);
    true
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR) and c:IsCanBeSpecialSummoned(e,122,tp,false,false)
    card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && card::is_can_be_special_summoned(d, c, e, 122, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        let c = effect::get_handler(d, a.e).nil();
        // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
        if card::get_sequence(d, c) < 5 {
            ft += 1;
        }
        // return ft>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,2,nil,e,tp)
        return ft > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=2 then
    if g.len() >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,2,2,nil)
        let sg = group::select(d, &g, a.tp, 2, 2, false, NO_EX).nil();
        // local tc=sg:GetFirst()
        let order = group::iter(d, &sg);
        let tc = order[0];
        // Duel.SpecialSummonStep(tc,122,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 122, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // tc:RegisterFlagEffect(tc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
        let code = card::get_original_code(d, tc);
        card::register_flag_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        // tc=sg:GetNext()
        let tc = order[1];
        // Duel.SpecialSummonStep(tc,122,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 122, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // tc:RegisterFlagEffect(tc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
        let code = card::get_original_code(d, tc);
        card::register_flag_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
