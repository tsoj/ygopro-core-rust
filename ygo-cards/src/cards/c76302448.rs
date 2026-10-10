//! Arcana Spread
use super::prelude::*;

pub const ID: u32 = 76302448;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, listed_names: &[CARD_LIGHT_BARRIER], listed_series: &[SET_ARCANA_FORCE as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin and apply an effect based on the result
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COIN+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 card from your GY to your hand that has a coin tossing effect
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
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.headsspfilter(c,e,tp)`
fn headsspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_ARCANA_FORCE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tailsspfilter(c,e,tp)`
fn tailsspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c.toss_coin and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    crate::card_info(d, c).toss_coin && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and (Duel.IsExistingMatchingCard(s.headsspfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    //     or Duel.IsExistingMatchingCard(s.tailsspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && (duel::is_existing_matching_card(d, Some(&|d, c| headsspfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
                || duel::is_existing_matching_card(d, Some(&|d, c| tailsspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX));
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local coin=nil
    let coin: Option<i32>;
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_LIGHT_BARRIER),tp,LOCATION_FZONE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_LIGHT_BARRIER]))), tp, LOCATION_FZONE, 0, 1, NO_EX) {
        // local b1=Duel.IsExistingMatchingCard(s.headsspfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        let b1 = duel::is_existing_matching_card(d, Some(&|d, c| headsspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
        // local b2=Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.tailsspfilter),tp,LOCATION_GRAVE,0,1,nil,e,tp)
        let nv = aux::necro_valley_filter(|d, c| tailsspfilter(d, c, e, tp));
        let b2 = duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_GRAVE, 0, 1, NO_EX);
        // local op=Duel.SelectEffect(tp,
        //     {b1,aux.GetCoinEffectHintString(COIN_HEADS)},
        //     {b2,aux.GetCoinEffectHintString(COIN_TAILS)})
        let op = duel::select_effect(
            d,
            tp,
            &[
                (b1, aux::get_coin_effect_hint_string(COIN_HEADS as i32).nil()),
                (b2, aux::get_coin_effect_hint_string(COIN_TAILS as i32).nil()),
            ],
        );
        // if not op then return end
        let Some(op) = op else {
            return;
        };
        // coin=op==1 and COIN_HEADS or COIN_TAILS
        coin = Some(if op == 1 { COIN_HEADS as i32 } else { COIN_TAILS as i32 });
    // else
    } else {
        // coin=Duel.TossCoin(tp,1)
        coin = Some(duel::toss_coin(d, tp, 1)[0]);
    }
    // if coin==COIN_HEADS then
    if coin == Some(COIN_HEADS as i32) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.headsspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| headsspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif coin==COIN_TAILS then
    } else if coin == Some(COIN_TAILS as i32) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tailsspfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let nv = aux::necro_valley_filter(|d, c| tailsspfilter(d, c, e, tp));
        let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c.toss_coin and c:IsAbleToHand()
    crate::card_info(d, c).toss_coin && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
