//! Attack Guidance Armor
use super::prelude::*;

pub const ID: u32 = 3103067;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy the attacking monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_label(d, e1, &[0]);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Change the attack target
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, 0);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_label(d, e2, &[1]);
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let label = effect::get_label(d, a.e);
    let bc = duel::get_attacker(d);
    // Group.FromCards(bc)
    let mut excg = match bc {
        Some(bc) => CardSet::from_card(bc),
        None => CardSet::new(),
    };
    // if dc then excg:AddCard(dc) end
    if let Some(dc) = duel::get_attack_target(d) {
        group::add_card(&mut excg, &CardSet::from_card(dc));
    }
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && !group::is_contains(&excg, chkc);
    }
    if a.chk == Some(0) {
        if label == 0 {
            return card::is_on_field(d, bc.nil());
        } else {
            return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, (&excg).into());
        }
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, aux::stringid(ID, label as u32));
    if label == 0 {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACKTARGET as u64);
        duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, (&excg).into());
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let label = effect::get_label(d, a.e);
    let bc = duel::get_attacker(d);
    let tc = duel::first_target(d);
    if label == 0 && card::is_relate_to_battle(d, bc.nil()) {
        duel::destroy(d, bc.nil(), REASON_EFFECT, None, None);
    } else if label == 1 && card::can_attack(d, bc.nil()) && !card::is_immune_to_effect(d, bc.nil(), a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                duel::calculate_damage(d, bc.nil(), Some(tc), false);
            }
        }
    }
}
