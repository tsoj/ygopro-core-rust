//! The Sun God's Poet - Helpoemer
use super::prelude::*;

pub const ID: u32 = 101403001;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID, CARD_THE_IMMORTAL_SUN_GOD],
    listed_series: &[SET_SUN_GOD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can discard 1 other card; Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::discard(None, true, None, None, None)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned: You can add 1 "Sun God" card from your Deck to your hand, except "The Sun God's Poet - Helpoemer", then if your opponent's LP are at least 2000 higher than yours, you can discard 1 random card from their hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // During the End Phase, if "The Immortal Sun God" was sent to your GY this turn: You can add this card from your GY to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| duel::has_flag_effect(d, a.tp, ID, None))));
    effect::set_target(d, e3, Some(cost_target(retthtg)));
    effect::set_operation(d, e3, Some(operation(retthop)));
    card::register_effect(d, c, e3, false);
    // Check if "The Immortal Sun God" is sent to the GY
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) for tc in eg:Iter() do ... end end)
        effect::set_operation(d, ge1, Some(operation(ge1_op)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// The global check's operation: `for tc in eg:Iter() do if tc:IsCode(CARD_THE_IMMORTAL_SUN_GOD) then Duel.RegisterFlagEffect(tc:GetControler(),id,RESET_PHASE|PHASE_END,0,1) end end`
fn ge1_op(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        if card::is_code(d, tc, &[CARD_THE_IMMORTAL_SUN_GOD]) {
            let p = card::get_controler(d, tc);
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    // if c:IsRelateToEffect(e) then Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SUN_GOD) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_SUN_GOD as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 1);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // if (Duel.GetLP(1-tp)-Duel.GetLP(tp))>=2000 and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::get_lp(d, 1 - tp) - duel::get_lp(d, tp) >= 2000
            && duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // local sg=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(1-tp,1)
            let hand = duel::get_field_group(d, tp, 0, LOCATION_HAND);
            let sg = group::random_select(d, &hand, 1 - tp, 1);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
        }
    }
}
// s.retthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn retthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// s.retthop(e,tp,eg,ep,ev,re,r,rp)
fn retthop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SendtoHand(c,nil,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
