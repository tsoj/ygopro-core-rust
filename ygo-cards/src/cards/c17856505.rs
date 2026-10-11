//! Pumpking the Great Ghost King
//! ゴースト大王－パンプキング－
use super::prelude::*;

pub const ID: u32 = 17856505;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CALL_OF_THE_HAUNTED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 6 Zombie monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_ZOMBIE),6,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_ZOMBIE, sc, st, Some(tp)))), Some(6), 2, None, None, None, None, false, None);
    // While you control "Call of the Haunted", your opponent cannot target Zombie monsters you control with monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_ZOMBIE))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_ZOMBIE))));
    // e1:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_CALL_OF_THE_HAUNTED),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetValue(function(e,te) return e:GetHandlerPlayer()==1-te:GetHandlerPlayer() and te:IsMonsterEffect() end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| value1(d, a.e(0), a.e(1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned: You can add 1 "Call of the Haunted" or 1 card that mentions it from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.deckthtg)
    effect::set_target(d, e2, Some(cost_target(deckthtg)));
    // e2:SetOperation(s.deckthop)
    effect::set_operation(d, e2, Some(operation(deckthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can detach 1 material from this card, then target up to 2 cards on the field; return them to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.rthtg)
    effect::set_target(d, e3, Some(cost_target(rthtg)));
    // e3:SetOperation(s.rthop)
    effect::set_operation(d, e3, Some(operation(rthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_CALL_OF_THE_HAUNTED),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_CALL_OF_THE_HAUNTED])), hp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `function(e,te) return e:GetHandlerPlayer()==1-te:GetHandlerPlayer() and te:IsMonsterEffect() end`
fn value1(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    let hp = effect::get_handler_player(d, e);
    let tp = effect::get_handler_player(d, te);
    hp == 1 - tp && effect::is_monster_effect(d, te)
}
/// `s.deckthfilter(c)`
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_CALL_OF_THE_HAUNTED) or c:ListsCode(CARD_CALL_OF_THE_HAUNTED)) and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_CALL_OF_THE_HAUNTED]) || card::lists_code(d, c, &[CARD_CALL_OF_THE_HAUNTED])) && card::is_able_to_hand(d, c, None)
}
/// `s.deckthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.deckthop(e,tp,eg,ep,ev,re,r,rp)`
fn deckthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, tp, 0);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
