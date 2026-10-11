//! Evigishki Tetrogre
use super::prelude::*;

pub const ID: u32 = 21496848;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GISHKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Send to the GY 1 card from the opponent's Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 and Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 && duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,5))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 5));
    // local ac=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2),aux.Stringid(id,3))
    let ac = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2), aux::stringid(ID, 3)]);
    // e:SetLabel(ac)
    effect::set_label(d, a.e, &[ac as i64]);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 and Duel.IsChainDisablable(0)
    //   and Duel.SelectYesNo(1-tp,aux.Stringid(id,4)) then
    if duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0
        && duel::is_chain_disablable(d, 0)
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 4))
    {
        // Duel.DiscardHand(1-tp,aux.TRUE,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, 1 - tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        return;
    }
    // local ty=TYPE_MONSTER
    // if e:GetLabel()==1 then ty=TYPE_SPELL elseif e:GetLabel()==2 then ty=TYPE_TRAP end
    let ty = match effect::get_label(d, a.e) {
        1 => TYPE_SPELL,
        2 => TYPE_TRAP,
        _ => TYPE_MONSTER,
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,Card.IsType,tp,LOCATION_DECK,0,1,1,nil,ty)
    let mut g1 = duel::select_matching_card(d, tp, Some(&|d, c| card::is_type(d, c, ty)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.SelectMatchingCard(1-tp,Card.IsType,1-tp,LOCATION_DECK,0,1,1,nil,ty)
    let g2 = duel::select_matching_card(d, 1 - tp, Some(&|d, c| card::is_type(d, c, ty)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SendtoGrave(g1,REASON_EFFECT)
    duel::send_to_grave(d, &g1, REASON_EFFECT, None, None);
}
