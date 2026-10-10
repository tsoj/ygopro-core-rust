//! Ancient Telescope
use super::prelude::*;

pub const ID: u32 = 17092736;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // confirm
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(cftg)));
    effect::set_operation(d, e1, Some(operation(cfop)));
    card::register_effect(d, c, e1, false);
}
fn cftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    true
}
fn cfop(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=math.min(5,Duel.GetFieldGroupCount(p,0,LOCATION_DECK))
    let ct = 5.min(duel::get_field_group_count(d, p, 0, LOCATION_DECK));
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_CONFIRM as u64);
    // local ac=Duel.AnnounceLevel(p,1,ct)
    let ac = duel::announce_number_range(d, p, Some(1), Some(ct as u32), &[]).nil().0;
    // local g=Duel.GetDecktopGroup(1-p,ac)
    let g = duel::get_decktop_group(d, 1 - p, ac as u32);
    // if #g>0 then Duel.ConfirmCards(p,g) end
    if !g.is_empty() {
        duel::confirm_cards(d, p, g);
    }
}
