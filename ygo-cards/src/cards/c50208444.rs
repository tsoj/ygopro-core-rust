//! Elemental Saga Doriado
use super::prelude::*;

pub const ID: u32 = 50208444;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Neither player can activate cards or effects in response to the activation of this card's effects
    // If you have 3 or more different Attributes in your GY and this card is in your hand: You can target 1 monster in your GY; Special Summon both it and this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,...) return Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil):GetBinClassCount(Card.GetAttribute)>=3 end)
    effect::set_condition(d, e1, Some(condition(attr_condition)));
    // e1:SetTarget(function(e,tp,...,chk,chkc) ... end)
    effect::set_target(d, e1, Some(cost_target(e1_target)));
    // e1:SetOperation(function(e,tp,...) ... end)
    effect::set_operation(d, e1, Some(operation(e1_operation)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you have 3 or more different Attributes in your GY: You can Special Summon 1 monster from your Deck with a different Attribute from the monsters in your field and GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,...) ... end)
    effect::set_condition(d, e2, Some(condition(attr_condition)));
    // e2:SetTarget(function(e,tp,...,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(function(e,tp,...) ... end)
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,...) return Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil):GetBinClassCount(Card.GetAttribute)>=3 end`
fn attr_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    group::get_bin_class_count(d, &g, &|d: &mut Duel, c: CardId| card::get_attribute(d, c) as i64) >= 3
}
/// `s.spcheck`-like helper: `c:IsCanBeSpecialSummoned(e,0,tp,false,false)`
fn can_ss(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `e1` target: `function(e,tp,...,chk,chkc)`
fn e1_target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && can_ss(d, chkc, e, tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,0,1,nil,e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && can_ss(d, c, e, tp)
            && duel::is_existing_target(d, Some(&|d, cc| can_ss(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,0,1,1,nil,e,0,tp,false,false)
    let g = duel::select_target(d, tp, Some(&|d, cc| can_ss(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g+c,2,tp,0)
    let mut gc = g.clone();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(gc), 2, tp, 0);
    // Duel.SetChainLimit(aux.FALSE)
    duel::set_chain_limit(d, aux_false());
    true
}
/// `e1` operation: `function(e,tp,...)`
fn e1_operation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
    if card::is_relate_to_effect(d, c, e)
        && card::is_relate_to_effect(d, tc, e)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && can_ss(d, c, e, tp)
        && can_ss(d, tc, e, tp)
    {
        // Duel.SpecialSummon(Group.FromCards(c,tc),0,tp,tp,false,false,POS_FACEUP)
        let mut sg = CardSet::from_card(c);
        group::add_card(&mut sg, &CardSet::from_card(tc));
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `e2` target: `function(e,tp,...,chk)`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetChainLimit(aux.FALSE)
    duel::set_chain_limit(d, aux_false());
    true
}
/// `e2` operation: `function(e,tp,...)`
fn e2_operation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| deckspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.deckspfilter(c,e,tp)`
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,c:GetAttribute()),tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    let att = card::get_attribute(d, c);
    can_ss(d, c, e, tp)
        && !duel::is_existing_matching_card(
            d,
            Some(&|d, cc| card::is_faceup(d, cc) && card::is_attribute(d, cc, att)),
            tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
}
