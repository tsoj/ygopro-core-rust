//! Peeking Goblin
use super::prelude::*;

pub const ID: u32 = 52263685;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>=3 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) >= 3;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // if Duel.GetFieldGroupCount(p,0,LOCATION_DECK)==0 then return end
    if duel::get_field_group_count(d, p, 0, LOCATION_DECK) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(1-p,3)
    let _ = duel::confirm_decktop(d, 1 - p, 3);
    // local g=Duel.GetDecktopGroup(1-p,3)
    let g = duel::get_decktop_group(d, 1 - p, 3);
    // Duel.Hint(HINT_SELECTMSG,p,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, p, aux::stringid(ID, 0));
    // local sg=g:Select(p,1,1,nil)
    let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
    // Duel.MoveToDeckBottom(sg)
    duel::move_to_deck_bottom(d, sg.clone());
    // Duel.ConfirmCards(1-p,sg)
    duel::confirm_cards(d, 1 - p, sg);
    // Duel.SortDecktop(p,1-p,2)
    duel::sort_decktop(d, p, 1 - p, 2);
    let _ = a;
}
