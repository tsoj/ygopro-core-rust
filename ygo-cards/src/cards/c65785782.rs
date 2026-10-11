//! Atlantis Call
use super::prelude::*;

pub const ID: u32 = 65785782;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON, CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 monster that mentions "Atlantis, the Dragon City" from your Deck to your hand, then if you control "Atlantis, the Dragon City", you can negate the effects of 1 Effect Monster your opponent controls until the end of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you control "Umi": You can banish this card from your GY; all WATER monsters you currently control gain 500 ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_UMI),tp,LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(...)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(...)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`: `c:ListsCode(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON) and c:IsMonster() and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::lists_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `aux.FaceupFilter(Card.IsCode,CARD_UMI)`
fn faceup_umi(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_UMI])
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_UMI),tp,LOCATION_ONFIELD,0,1,nil) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&faceup_umi), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)`: `aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER)` on the field
fn faceup_water(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&faceup_water), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,nil,1,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, a.tp, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DEFCHANGE,nil,1,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, None, 1, a.tp, 500);
    true
}
fn e2_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&faceup_water), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:UpdateAttack(500,nil,c)
        card::update_attack(d, tc, 500, None, Some(c), None);
        // tc:UpdateDefense(500,nil,c)
        card::update_defense(d, tc, 500, None, Some(c), None);
    }
}
/// `s.target`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.activate`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON),tp,LOCATION_ONFIELD,0,1,nil)
        //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsEffectMonster,Card.IsNegatableMonster),tp,0,LOCATION_MZONE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&faceup_atlantis), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&negatable_effect_monster), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
            // local sc=Duel.SelectMatchingCard(tp,aux.AND(Card.IsEffectMonster,Card.IsNegatableMonster),tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
            let sg = duel::select_matching_card(d, a.tp, Some(&negatable_effect_monster), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            // if not sc then return end
            let sc = match group::get_first(d, &sg) {
                Some(sc) => sc,
                None => return,
            };
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // sc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END)
            let h = effect::get_handler(d, a.e).nil();
            card::negate_effects(d, sc, h, Some(RESET_PHASE | PHASE_END), false, None);
        }
    }
}
/// `aux.FaceupFilter(Card.IsCode,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON)`
fn faceup_atlantis(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON])
}
/// `aux.AND(Card.IsEffectMonster,Card.IsNegatableMonster)`
fn negatable_effect_monster(d: &mut Duel, c: CardId) -> bool {
    card::is_effect_monster(d, c) && card::is_negatable_monster(d, c)
}
