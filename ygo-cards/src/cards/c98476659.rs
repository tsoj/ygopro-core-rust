//! Pot of Sloth
use super::prelude::*;

pub const ID: u32 = 98476659;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw cards equal to the number of cards your opponent controls, then ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW | CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local draw_count=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let draw_count = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) as u32;
    // if chk==0 then return draw_count>0 and Duel.IsPlayerCanDraw(tp,draw_count) end
    if a.chk == Some(0) {
        return draw_count > 0 && duel::is_player_can_draw(d, a.tp, draw_count);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,draw_count)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, draw_count as i32);
    // if draw_count>=2 then
    if draw_count >= 2 {
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,draw_count-1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, (draw_count - 1) as u8, a.tp, LOCATION_HAND as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local draw_count=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let draw_count = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) as u32;
    // if draw_count>0 and Duel.Draw(tp,draw_count,REASON_EFFECT)>0 and draw_count>=2 then
    if draw_count > 0 && duel::draw(d, a.tp, draw_count, REASON_EFFECT) > 0 && draw_count >= 2 {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // local return_count=draw_count-1
        let return_count = draw_count - 1;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,return_count,return_count,nil)
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&card::is_able_to_deck),
            a.tp,
            LOCATION_HAND,
            0,
            return_count as u16,
            return_count as u16,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and return_count>=2 then
            if duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 && return_count >= 2 {
                // Duel.SortDeckbottom(tp,tp,return_count)
                duel::sort_deckbottom(d, a.tp, a.tp, return_count as u16);
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(e,re) return re:GetHandler():IsCode(id) end)
    effect::set_value_fn(d, e1, lua_fn(value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `function(e,re) return re:GetHandler():IsCode(id) end`
fn value(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(1)).nil();
    card::is_code(d, h, &[ID])
}
