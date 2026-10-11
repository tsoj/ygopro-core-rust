//! Gishki Noellia
//! リチュア・ノエリア
use super::prelude::*;

pub const ID: u32 = 63942761;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GISHKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Mill
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,5) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 5);
    }
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSpell() or (c:IsSetCard(SET_GISHKI) and c:IsMonster())
    card::is_ritual_spell(d, c) || (card::is_set_card(d, c, &[SET_GISHKI as u16]) && card::is_monster(d, c))
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerCanDiscardDeck(tp,5) then return end
    if !duel::is_player_can_discard_deck(d, tp, 5) {
        return;
    }
    // Duel.ConfirmDecktop(tp,5)
    duel::confirm_decktop(d, tp, 5);
    // local g=Duel.GetDecktopGroup(tp,5)
    let g = duel::get_decktop_group(d, tp, 5);
    // local sg=g:Filter(s.filter,nil)
    let sg = group::filter(d, &g, &filter, NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, &sg, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
    // Duel.MoveToDeckBottom(5-#sg,tp)
    duel::move_decktop_to_bottom(d, (5 - sg.len()) as u32, tp);
    // Duel.SortDeckbottom(tp,tp,5-#sg)
    duel::sort_deckbottom(d, tp, tp, (5 - sg.len()) as u16);
}
