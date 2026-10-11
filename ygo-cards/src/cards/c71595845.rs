//! Doll Happiness
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 71595845;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DOLL_MONSTER],
    listed_names: &[CARD_GRANDPA_DEMETTO, CARD_BOX_OF_FRIENDS, CARD_PRINCESS_COLOGNE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is Activated: Add 1 "Grandpa Demetto" or "Box of Friends" from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // While you control "Princess Cologne", your opponent's monsters cannot target monsters with 0 ATK or DEF for attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_PRINCESS_COLOGNE),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetValue(function(e,c) return (c:IsAttack(0) or c:IsDefense(0)) and c:IsFaceup() end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, _e, c| {
        (card::is_attack(d, c, &[0]) || card::is_defense(d, c, &[0])) && card::is_faceup(d, c)
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 monster in your hand or field, and if you do, send 1 "Doll Monster" card from your Deck to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_PRINCESS_COLOGNE),...) end
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_PRINCESS_COLOGNE])),
        p,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_GRANDPA_DEMETTO,CARD_BOX_OF_FRIENDS) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_GRANDPA_DEMETTO, CARD_BOX_OF_FRIENDS]) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DOLL_MONSTER) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_DOLL_MONSTER as u16]) && card::is_able_to_grave(d, c, None)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsMonster,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_monster), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local desg=Duel.SelectMatchingCard(tp,Card.IsMonster,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let desg = duel::select_matching_card(d, tp, Some(&card::is_monster), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #desg>0 and Duel.Destroy(desg,REASON_EFFECT)>0 then
    if !desg.is_empty() && duel::destroy(d, desg, REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local gyg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let gyg = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #gyg>0 then
        if !gyg.is_empty() {
            // Duel.SendtoGrave(gyg,REASON_EFFECT)
            duel::send_to_grave(d, gyg, REASON_EFFECT, None, None);
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function (e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_XYZ) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| {
        card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_XYZ)
    })));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // "Clock Lizard" check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalType(TYPE_XYZ) end)
    aux::add_temp_lizard_check(
        d,
        c,
        tp,
        Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_XYZ))),
        None,
        None,
        None,
        None,
    );
}
