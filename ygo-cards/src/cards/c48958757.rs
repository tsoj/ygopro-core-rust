//! Gladiator Beast Claudius
use super::prelude::*;

pub const ID: u32 = 48958757;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 5 "Gladiator Beast" monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GLADIATOR_BEAST),5)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GLADIATOR_BEAST as u16], Some(fc), st, Some(tp)))),
            5,
        )],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,function(e) return not e:GetHandler():IsLocation(LOCATION_EXTRA) end,nil,1)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(sumcon))),
            sumtype: Some(1),
            ..Default::default()
        },
    );
    // Apply a "you can conduct your next Battle Phase twice" effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e,tp) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) and not Duel.IsPlayerAffectedByEffect(tp,EFFECT_BP_TWICE) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetOperation(s.doublebattlephase)
    effect::set_operation(d, e1, Some(operation(doublebattlephase)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate the appropriate effect, depending on whose turn it is
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re) return ep==1-tp and re:IsMonsterEffect() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(aux.AND(Card.IsMonster,Card.IsAbleToDeckOrExtraAsCost),tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck_or_extra_as_cost(d, c)),
        tp,
        LOCATION_MZONE | LOCATION_GRAVE,
        0,
        NO_EX,
    )
}
/// `s.contactop(g,tp,c)`
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // local fdg=g:Filter(Card.IsFacedown,nil)
    let fdg = group::filter(d, g, &|d, c| card::is_facedown(d, c), NO_EX);
    // local gyg=g:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let gyg = group::filter(d, g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #fdg>0 then Duel.ConfirmCards(1-tp,fdg) end
    if !fdg.is_empty() {
        duel::confirm_cards(d, 1 - tp, fdg);
    }
    // if #gyg>0 then Duel.HintSelection(gyg) end
    if !gyg.is_empty() {
        duel::hint_selection(d, gyg, true);
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
/// `function(e,tp) return not e:GetHandler():IsLocation(LOCATION_EXTRA) end` (the summon condition of the contact procedure)
fn sumcon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, c, LOCATION_EXTRA)
}
/// `function(e,tp,eg,ep,ev,re) ... end` of e1 (`function(e,tp)` in the Lua)
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) and not Duel.IsPlayerAffectedByEffect(tp,EFFECT_BP_TWICE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1]) && duel::is_player_affected_by_effect(d, a.tp, EFFECT_BP_TWICE).is_none()
}
/// `function(e,tp,eg,ep,ev,re) return ep==1-tp and re:IsMonsterEffect() end` (the condition of e2)
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
/// `s.doublebattlephase(e,tp,eg,ep,ev,re,r,rp)`
fn doublebattlephase(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,EFFECT_BP_TWICE) then return end
    if duel::is_player_affected_by_effect(d, a.tp, EFFECT_BP_TWICE).is_some() {
        return;
    }
    // local turn_ct=Duel.GetTurnCount()
    let turn_ct = duel::get_turn_count(d, None);
    // local ct=Duel.IsTurnPlayer(tp) and Duel.IsBattlePhase() and 2 or 1
    let ct: i64 = if duel::is_turn_player(d, a.tp) && duel::is_battle_phase(d, None) { 2 } else { 1 };
    // --You can conduct your next Battle Phase twice
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_BP_TWICE)
    effect::set_code(d, e1, EFFECT_BP_TWICE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetCondition(function() return ct==1 or Duel.GetTurnCount()~=turn_ct end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(move |d, _a: Args| ct == 1 || duel::get_turn_count(d, None) != turn_ct)),
    );
    // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_SELF_TURN,ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_SELF_TURN, ct as u16);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.deckspfilter(c,e,tp)`
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and c:IsCanBeSpecialSummoned(e,100,tp,false,false)
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 100, tp, false, false, None, None, None)
}
/// `s.extraspfilter(c,e,tp)`
fn extraspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(11) and c:IsSetCard(SET_GLADIATOR_BEAST) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,100,tp,true,false)
    card::is_level_below(d, c, 11)
        && card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 100, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsTurnPlayer(tp) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    let b1 = duel::is_turn_player(d, a.tp)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsTurnPlayer(1-tp)
    //     and Duel.IsExistingMatchingCard(s.extraspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    let b2 = duel::is_turn_player(d, 1 - a.tp)
        && duel::is_existing_matching_card(d, Some(&|d, c| extraspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=b1 and 1 or 2
    let op: i64 = if b1 { 1 } else { 2 };
    // local sum_loc=b1 and LOCATION_DECK or LOCATION_EXTRA
    let sum_loc = if b1 { LOCATION_DECK } else { LOCATION_EXTRA };
    // e:SetLabel(op,sum_loc)
    effect::set_label(d, a.e, &[op, sum_loc as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,sum_loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, sum_loc as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local op,sum_loc=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let op = labels.first().copied().unwrap_or(0);
    let sum_loc = labels.get(1).copied().unwrap_or(0) as u32;
    // if op==1 and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if op == 1 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local spfilter=op==1 and s.deckspfilter or s.extraspfilter
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,spfilter,tp,sum_loc,0,1,1,nil,e,tp):GetFirst()
    let g = if op == 1 {
        duel::select_matching_card(d, a.tp, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, sum_loc, 0, 1, 1, false, NO_EX)
    } else {
        duel::select_matching_card(d, a.tp, Some(&|d, c| extraspfilter(d, c, a.e, a.tp)), a.tp, sum_loc, 0, 1, 1, false, NO_EX)
    }
    .nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.SpecialSummon(tc,100,tp,tp,op==2,false,POS_FACEUP)
        duel::special_summon(d, tc, 100, a.tp, a.tp, op == 2, false, POS_FACEUP as u8, None);
        // tc:RegisterFlagEffect(tc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,0)
        let code = card::get_original_code(d, tc);
        card::register_flag_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
    }
}
