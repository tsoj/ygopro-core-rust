//! Dark Doriado
use super::prelude::*;

pub const ID: u32 = 62312469;

/// `local ATTRIBUTES=ATTRIBUTE_EARTH|ATTRIBUTE_WATER|ATTRIBUTE_FIRE|ATTRIBUTE_WIND`
const ATTRIBUTES: u32 = ATTRIBUTE_EARTH | ATTRIBUTE_WATER | ATTRIBUTE_FIRE | ATTRIBUTE_WIND;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetValue(s.value)
    effect::set_value_fn(d, e1, effect_card_value(value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetTarget(s.sttg)
    effect::set_target(d, e3, Some(cost_target(sttg)));
    // e3:SetOperation(s.stop)
    effect::set_operation(d, e3, Some(operation(stop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTES)
    card::is_attribute(d, c, ATTRIBUTES)
}
/// `s.value(e,c)`
fn value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // return Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil):GetBinClassCount(Card.GetAttribute)*200
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    group::get_bin_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) as i64 * 200
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAttribute(ATTRIBUTES)
    card::is_monster(d, c) && card::is_attribute(d, c, ATTRIBUTES)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return true,not sg:CheckDifferentPropertyBinary(function(c)return c:GetAttribute()&(ATTRIBUTES)end)
    (true, !group::check_different_property_binary(d, sg, &|d, c| (card::get_attribute(d, c) & ATTRIBUTES) as u64))
}
/// `s.sttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return #g>=4 and aux.SelectUnselectGroup(g,e,tp,4,4,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 4
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(4), Some(4), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    true
}
/// `s.stop(e,tp,eg,ep,ev,re,r,rp)`
fn stop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c)), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,4,4,s.rescon,1,tp,HINTMSG_TARGET)
    // (not cancelable: the selection is always a group, so `if sg then` always holds)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(4), Some(4), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg.clone());
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // Duel.MoveToDeckTop(sg)
    duel::move_to_deck_top(d, sg);
    // Duel.SortDecktop(tp,tp,4)
    duel::sort_decktop(d, tp, tp, 4);
}
