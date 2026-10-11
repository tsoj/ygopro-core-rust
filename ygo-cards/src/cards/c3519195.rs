//! Evil Assault
use super::prelude::*;

pub const ID: u32 = 3519195;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DARK_FUSION],
    listed_series: &[SET_EVIL_HERO, SET_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take 1 Level 4 or lower "Evil HERO" monster from your Deck, and either add it to your hand or Special Summon it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thspcost)
    effect::set_cost(d, e1, Some(cost_target(thspcost)));
    // e1:SetTarget(s.thsptg)
    effect::set_target(d, e1, Some(cost_target(thsptg)));
    // e1:SetOperation(s.thspop)
    effect::set_operation(d, e1, Some(operation(thspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Dark Fusion" from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_discardable(d, c, None, None)),
            a.tp,
            LOCATION_HAND,
            0,
            1,
            h.into(),
        );
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.thspfilter(c,e,tp,zone_chk)
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone_chk: bool) -> bool {
    // return c:IsSetCard(SET_EVIL_HERO) and c:IsLevelBelow(4) and (c:IsAbleToHand() or (zone_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_EVIL_HERO as u16])
        && card::is_level_below(d, c, 4)
        && (card::is_able_to_hand(d, c, None)
            || (zone_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let zone_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,zone_chk)
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| thspfilter(d, c, a.e, a.tp, zone_chk)),
            a.tp,
            LOCATION_DECK,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thspop(e,tp,eg,ep,ev,re,r,rp)
fn thspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local zone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let zone_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,zone_chk):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| thspfilter(d, c, a.e, tp, zone_chk)),
        tp,
        LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // aux.ToHandOrElse(sc,tp,
        //   function(c) return zone_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //   function(c) Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
        //   aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            sc,
            tp,
            Some(&|d, _c| zone_chk && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None)),
            Some(&|d, _g| {
                duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 3)),
        );
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn after this card resolves, except "HERO" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSetCard(SET_HERO) end)
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_set_card(d, c, &[SET_HERO as u16]))),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalSetCard(SET_HERO) end)
    aux::add_temp_lizard_check(
        d,
        c,
        tp,
        Some(effect_card_fn(|d, _e, c| !card::is_original_set_card(d, c, &[SET_HERO as u16]))),
        None,
        None,
        None,
        None,
    );
}
// function s.gythfilter(c)
fn gythfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_DARK_FUSION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_DARK_FUSION]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gythfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gythfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gythfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&gythfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
