//! Royal Tribute
use super::prelude::*;

pub const ID: u32 = 72405967;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NECROVALLEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(handestg)));
    effect::set_operation(d, e1, Some(operation(handesop)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e,tp) return Duel.IsEnvironment(CARD_NECROVALLEY,tp) end
    duel::is_environment(d, CARD_NECROVALLEY, Some(a.tp), None)
}
fn handestg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_HAND,LOCATION_HAND,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_HAND, LOCATION_HAND, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_HAND as i32);
    true
}
fn handesop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_HAND,LOCATION_HAND,nil)
    let g = duel::get_matching_group(d, Some(&card::is_monster), a.tp, LOCATION_HAND, LOCATION_HAND, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, &g, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
