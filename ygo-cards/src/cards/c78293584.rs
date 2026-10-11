//! Filia Diabell
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 78293584;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DIABELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Level 8 or higher "Diabell" monster from your Deck to your hand, or, if only your opponent controls a monster, you can Special Summon it instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thsptg)
    effect::set_target(d, e1, Some(cost_target(thsptg)));
    // e1:SetOperation(s.thspop)
    effect::set_operation(d, e1, Some(operation(thspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make all "Diabell" monsters you currently control gain 500 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thspfilter(c,e,tp,sp_check)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sp_check: bool) -> bool {
    // return c:IsSetCard(SET_DIABELL) and c:IsLevelAbove(8)
    //     and (c:IsAbleToHand() or (sp_check and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_DIABELL as u16])
        && card::is_level_above(d, c, 8)
        && (card::is_able_to_hand(d, c, None) || (sp_check && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `local sp_check=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0`
fn sp_check_of(d: &mut Duel, tp: u8) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local sp_check=...
        let sp_check = sp_check_of(d, a.tp);
        // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,sp_check)
        return duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, sp_check)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND as u64, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON as u64, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sp_check=...
    let sp_check = sp_check_of(d, tp);
    // local desc=sp_check and aux.Stringid(id,2) or HINTMSG_ATOHAND
    let desc = if sp_check { aux::stringid(ID, 2) } else { HINTMSG_ATOHAND as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,desc)
    duel::hint(d, HINT_SELECTMSG as u8, tp, desc);
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,sp_check):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, a.e, tp, sp_check)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else { return };
    // if sp_check then
    if sp_check {
        // aux.ToHandOrElse(sc,tp, function() return sp_check and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end, aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            sc,
            tp,
            Some(&|d, _c| sp_check && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None)),
            Some(&|d, _g| {
                duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 3)),
        );
    } else {
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DIABELL),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_DIABELL as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,g,#g,tp,500)
    let n = g.len() as u8;
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE as u64, Some(g), n, a.tp, 500);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DIABELL),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_DIABELL as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
