//! Diabolos, King of the Abyss
use super::prelude::*;

pub const ID: u32 = 29424328;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // tribute limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRIBUTE_LIMIT)
    effect::set_code(d, e2, EFFECT_TRIBUTE_LIMIT);
    // e2:SetValue(s.tlimit)
    effect::set_value_fn(d, e2, effect_card_fn(|d, _e, c| tlimit(d, c)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // release limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UNRELEASABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_UNRELEASABLE_EFFECT);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // confirm deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e4, EVENT_PREDRAW);
    // e4:SetCondition(s.cfcon)
    effect::set_condition(d, e4, Some(condition(cfcon)));
    // e4:SetOperation(s.cfop)
    effect::set_operation(d, e4, Some(operation(cfop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tlimit(e,c)`
fn tlimit(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsAttribute(ATTRIBUTE_DARK)
    !card::is_attribute(d, c, ATTRIBUTE_DARK)
}
/// `s.cfcon(e,tp,eg,ep,ev,re,r,rp)`
fn cfcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetFieldGroupCount(1-tp,LOCATION_DECK,0)>0
    duel::is_turn_player(d, 1 - a.tp) && duel::get_field_group_count(d, 1 - a.tp, LOCATION_DECK, 0) > 0
}
/// `s.cfop(e,tp,eg,ep,ev,re,r,rp)`
fn cfop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(1-tp,1)
    let g = duel::get_decktop_group(d, 1 - tp, 1);
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g.clone());
    // local tc=g:GetFirst()
    // local opt=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    let opt = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // if opt==1 then
    if opt == 1 {
        // Duel.MoveSequence(tc,opt)
        let tc = group::get_first(d, &g).nil();
        duel::move_sequence(d, tc, opt as u8, None);
    }
}
