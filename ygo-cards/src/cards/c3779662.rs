//! Gladiator Beast Andabata
use super::prelude::*;

pub const ID: u32 = 3779662;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GLADIATOR],
    material_setcode: &[SET_GLADIATOR as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,7573135,1,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GLADIATOR),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::Code(7573135), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GLADIATOR as u16], Some(fc), st, Some(tp)))), 2),
        ],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit,aux.TRUE,1)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(|d, tp| contactfil(d, tp))),
            material_operation: Some(std::sync::Arc::new(|d, g, tp, _c| contactop(d, g, tp))),
            sumcon: Some(Some(lua_fn(splimit))),
            condition: Some(std::sync::Arc::new(|_d, _tp| true)),
            sumtype: Some(1),
            ..Default::default()
        },
    );
    // extra summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e3, aux::stringid(ID, 4));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.espcon)
    effect::set_condition(d, e3, Some(condition(espcon)));
    // e3:SetTarget(s.esptg)
    effect::set_target(d, e3, Some(cost_target(esptg)));
    // e3:SetOperation(s.espop)
    effect::set_operation(d, e3, Some(operation(espop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // special summon
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e6, aux::stringid(ID, 5));
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
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(function(c) return c:IsMonster() and c:IsAbleToDeckOrExtraAsCost() end,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck_or_extra_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.contactop(g,tp)`
fn contactop(d: &mut Duel, g: &CardSet, tp: u8) {
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetLocation()~=LOCATION_EXTRA
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_location(d, h) != LOCATION_EXTRA
}
/// `s.espcon(e,tp,eg,ep,ev,re,r,rp)`
fn espcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == (SUMMON_TYPE_SPECIAL + 1) as u32
}
/// `s.espfilter(c,e,tp)`
fn espfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR) and c:IsType(TYPE_FUSION) and c:IsLevelBelow(7)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,123,tp,true,false)
    card::is_set_card(d, c, &[SET_GLADIATOR as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_level_below(d, c, 7)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 123, tp, true, false, None, None, None)
}
/// `s.esptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn esptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.espfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| espfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.espop(e,tp,eg,ep,ev,re,r,rp)`
fn espop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.espfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| espfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SpecialSummon(g,123,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 123, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToExtraAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_extra_as_cost(d, c);
    }
    // Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_COST)
    duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR) and c:IsCanBeSpecialSummoned(e,123,tp,false,false)
    card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && card::is_can_be_special_summoned(d, c, e, 123, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
        let h = effect::get_handler(d, a.e).nil();
        if card::get_sequence(d, h) < 5 {
            ft += 1;
        }
        // return ft>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,2,nil,e,tp)
        return ft > 1
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT as u32).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT as u32).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=2 then
    if group::get_count(&g) >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,2,2,nil)
        let sg = group::select(d, &g, tp, 2, 2, false, NO_EX).nil();
        // local tc=sg:GetFirst()
        let tc = group::get_first(d, &sg).nil();
        // Duel.SpecialSummonStep(tc,123,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 123, tp, tp, false, false, POS_FACEUP as u8, None);
        // tc:RegisterFlagEffect(tc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
        let code = card::get_original_code(d, tc);
        card::register_flag_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        // tc=sg:GetNext()
        let tc = group::iter(d, &sg).get(1).copied().nil();
        // Duel.SpecialSummonStep(tc,123,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 123, tp, tp, false, false, POS_FACEUP as u8, None);
        // tc:RegisterFlagEffect(tc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
        let code = card::get_original_code(d, tc);
        card::register_flag_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
