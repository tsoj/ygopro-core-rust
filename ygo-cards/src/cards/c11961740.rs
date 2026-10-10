//! Different Dimension Capsule
use super::prelude::*;

pub const ID: u32 = 11961740;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_DECK,0,1,nil,tp,POS_FACEDOWN)
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && !card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        // if tc and Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)~=0 and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
        let mut removed = None;
        if let Some(tc) = tc {
            if duel::remove(d, tc, POS_FACEDOWN as u8, REASON_EFFECT, None, None) != 0 && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
                removed = Some(tc);
            }
        }
        if let Some(tc) = removed {
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_range(d, e1, LOCATION_SZONE);
            effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
            effect::set_condition(d, e1, Some(condition(thcon)));
            effect::set_operation(d, e1, Some(operation(thop)));
            effect::set_label(d, e1, &[0]);
            effect::set_label_object(d, e1, Value::Card(tc));
            card::register_effect(d, c, e1, false);
        } else {
            // c:CancelToGrave(false)
            card::cancel_to_grave(d, c, false);
        }
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let ct = effect::get_label(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    // e:GetHandler():SetTurnCounter(ct+1)
    card::set_turn_counter(d, h, (ct + 1) as u16);
    if ct == 1 {
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_RULE, None, None);
        let tc = effect::get_label_object(d, a.e).as_card().nil();
        if card::get_flag_effect(d, tc, ID) != 0 {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    } else {
        effect::set_label(d, a.e, &[1]);
    }
}
