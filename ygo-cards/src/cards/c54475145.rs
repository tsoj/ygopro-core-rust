//! Heart of the Blue-Eyes
use super::prelude::*;

pub const ID: u32 = 54475145;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_MILLENNIUM_CROSS],
    listed_series: &[SET_MILLENNIUM],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Millennium Ankh" from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Possession of this card cannot switch
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CANNOT_CHANGE_CONTROL)
    effect::set_code(d, e2, EFFECT_CANNOT_CHANGE_CONTROL);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1},EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e3, 1, ID, 1, EFFECT_COUNT_CODE_DUEL as u8);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Register that a player has activated "Millennium Ankh" during this Duel
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, ge1, EVENT_CHAIN_SOLVED);
        // ge1:SetOperation(s.regop)
        effect::set_operation(d, ge1, Some(operation(regop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.HasFlagEffect(rp,id) and re:GetHandler():IsCode(CARD_MILLENNIUM_CROSS) and re:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if duel::has_flag_effect(d, a.rp, ID, None) {
        return;
    }
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_code(d, h, &[CARD_MILLENNIUM_CROSS]) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        // Duel.RegisterFlagEffect(rp,id,0,0,0)
        duel::register_flag_effect(d, a.rp, ID, 0, 0, 0, 0);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_MILLENNIUM_CROSS) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_MILLENNIUM_CROSS]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and (c:IsLevelAbove(8) or c:IsAttackAbove(3000)) and c:IsFaceup() and c:IsAbleToGrave()
    //     and c:IsLocation(LOCATION_MZONE)
    card::is_summon_player(d, c, 1 - tp)
        && (card::is_level_above(d, c, 8) || card::is_attack_above(d, c, 3000))
        && card::is_faceup(d, c)
        && card::is_able_to_grave(d, c, None)
        && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.HasFlagEffect(tp,id) and eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    duel::has_flag_effect(d, a.tp, ID, None) && group::is_exists(d, &eg, &|d, c| spconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=eg:Filter(s.spconfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| spconfilter(d, x, a.tp), NO_EX);
    // if chk==0 then return Duel.GetMZoneCount(tp,g)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, a.tp, (&g).into(), None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g.clone()), g.len() as u8, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE)
    //     and c:IsRelateToEffect(e) then
    if !g.is_empty()
        && duel::send_to_grave(d, &g, REASON_EFFECT, None, None) > 0
        && group::is_exists(d, &g, &|d, x| card::is_location(d, x, LOCATION_GRAVE), 1, NO_EX)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
