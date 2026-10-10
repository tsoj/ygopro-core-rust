//! Drill Bug
use super::prelude::*;

pub const ID: u32 = 88733579;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[27911549], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.IsCode,tp,LOCATION_DECK,0,1,1,nil,27911549):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_code(d, x, &[27911549])), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.MoveToDeckTop(tc)
        duel::move_to_deck_top(d, tc);
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, tp, 1);
    }
}
