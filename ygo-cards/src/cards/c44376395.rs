//! ASHLAN U1000
use super::prelude::*;

pub const ID: u32 = 44376395;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 monsters with different Types and Attributes
    // Link.AddProcedure(c,nil,2,nil,s.spcheck)
    link::add_procedure(d, c, None, 2, None, Some(std::sync::Arc::new(spcheck)), None, None);
    // If a Ritual Monster this card points to attacks a Defense Position monster, inflict piercing battle damage to your opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(function(e,c) return c:IsRitualMonster() and e:GetHandler():GetLinkedGroup():IsContains(c) end)
    effect::set_target(d, e1, Some(effect_card_fn(pierce_target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can reveal 1 Ritual Monster in your hand; add 1 Ritual Monster with a different Type and Attribute from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If you Ritual Summon (except during the Damage Step): You can target 1 face-up card your opponent controls; return it to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.rthcon)
    effect::set_condition(d, e3, Some(condition(rthcon)));
    // e3:SetTarget(s.rthtg)
    effect::set_target(d, e3, Some(cost_target(rthtg)));
    // e3:SetOperation(s.rthop)
    effect::set_operation(d, e3, Some(operation(rthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcheck(g,lc,sumtype,tp)`
fn spcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckDifferentPropertyBinary(Card.GetRace,lc,sumtype,tp) and g:CheckDifferentPropertyBinary(Card.GetAttribute,lc,sumtype,tp)
    group::check_different_property_binary(d, g, &|d, c| card::get_race_ctx(d, c, Some(lc), sumtype, Some(tp)))
        && group::check_different_property_binary(d, g, &|d, c| card::get_attribute_ctx(d, c, Some(lc), sumtype, Some(tp)) as u64)
}
/// `function(e,c) return c:IsRitualMonster() and e:GetHandler():GetLinkedGroup():IsContains(c) end`
fn pierce_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    if !card::is_ritual_monster(d, c) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    let linked = card::get_linked_group(d, h);
    group::is_contains(&linked, c)
}
/// `s.thcostfilter(c,tp)`
fn thcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualMonster() and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetRace(),c:GetAttribute())
    if !(card::is_ritual_monster(d, c) && !card::is_public(d, c)) {
        return false;
    }
    let race = card::get_race(d, c);
    let attribute = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, race, attribute)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,race,attribute)`
fn thfilter(d: &mut Duel, c: CardId, race: u64, attribute: u32) -> bool {
    // return c:IsRitualMonster() and c:IsRaceExcept(race) and c:IsAttributeExcept(attribute) and c:IsAbleToHand()
    card::is_ritual_monster(d, c)
        && !card::is_race(d, c, race)
        && !card::is_attribute(d, c, attribute)
        && card::is_able_to_hand(d, c, None)
}
/// `s.thcost(e,tp,...,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thcostfilter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_HAND,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| thcostfilter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &sel).nil();
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(rc));
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabelObject(rc)
    effect::set_label_object(d, a.e, Value::Card(rc));
    // rc:CreateEffectRelation(e)
    card::create_effect_relation(d, rc, a.e);
    true
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, a.e).as_card().nil();
    // if not rc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, rc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,rc:GetRace(),rc:GetAttribute())
    let race = card::get_race(d, rc);
    let attribute = card::get_attribute(d, rc);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, race, attribute)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.rthconfilter(c,tp)`
fn rthconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualSummoned() and c:IsSummonPlayer(tp)
    card::is_ritual_summoned(d, c) && card::is_summon_player(d, c, tp)
}
/// `s.rthcon(e,tp,eg,...)`
fn rthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.rthconfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| rthconfilter(d, c, a.tp), 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsAbleToHand)`
fn faceup_tohand(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.rthtg(e,tp,...,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_tohand), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&faceup_tohand), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
/// `s.rthop(e,tp,...)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
