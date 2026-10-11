//! Earthbound Geoglyph
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 44710391;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND_IMMORTAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetCondition(s.indcon)
    effect::set_condition(d, e2, Some(condition(indcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // double tribute
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCode(EFFECT_DOUBLE_TRIBUTE)
    effect::set_code(d, e4, EFFECT_DOUBLE_TRIBUTE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(s.dttg)
    effect::set_target(d, e4, Some(effect_card_fn(dttg)));
    // e4:SetValue(s.dtcon)
    effect::set_value_fn(d, e4, effect_card_fn(dtcon));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // search
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCondition(s.thcon)
    effect::set_condition(d, e5, Some(condition(thcon)));
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}

/// `s.indcon(e)`: `Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevel,10),0,LOCATION_MZONE,LOCATION_MZONE,1,nil)`
fn indcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_level(d, c, &[10])), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}

/// `s.dttg(e,c)`: `c:IsType(TYPE_SYNCHRO) and (c:IsControler(e:GetHandlerPlayer() or c:IsFaceup()))`
/// (`e:GetHandlerPlayer()` is a number, so the `or` is never reached)
fn dttg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let hp = effect::get_handler_player(d, e);
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_controler(d, c, hp)
}

/// `s.dtcon(e,c)`: `c:IsControler(e:GetHandlerPlayer()) and c:IsSetCard(SET_EARTHBOUND_IMMORTAL)`
fn dtcon(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let hp = effect::get_handler_player(d, e);
    card::is_controler(d, c, hp) && card::is_set_card(d, c, &[SET_EARTHBOUND_IMMORTAL as u16])
}

/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`: `eg:IsExists(Card.IsType,1,nil,TYPE_SYNCHRO)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_type(d, c, TYPE_SYNCHRO), 1, NO_EX)
}

/// `s.thfilter(c)`: `c:IsSetCard(SET_EARTHBOUND_IMMORTAL) and c:IsSpellTrap() and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_EARTHBOUND_IMMORTAL as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}

/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
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
