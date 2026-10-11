//! Archfiend Matriarch
use super::prelude::*;

pub const ID: u32 = 78744660;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[63679166], listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    pendulum::add_procedure(d, c, None, None);
    card::add_must_be_ritual_summoned(d, c);
    // You can target 1 "Archfiend" Spell/Trap in your GY or banishment; destroy this card, and if you do, add the targeted card to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // If this card is Ritual Summoned: You can destroy as many Spells/Traps your opponent controls as possible, then if this card is in the Extra Monster Zone, you can banish all Spells and Traps from your opponent's GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Unaffected by other monsters' effects while in the Extra Monster Zone
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e3, LOCATION_EMZONE);
    effect::set_value_fn(d, e3, lua_fn(e3_value));
    card::register_effect(d, c, e3, false);
}
/// `function(e,te) return te:IsMonsterEffect() and te:GetOwner()~=e:GetOwner() end`
fn e3_value(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::is_monster_effect(d, te) && effect::get_owner(d, te).nil() != effect::get_owner(d, a.e(0)).nil()
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARCHFIEND as u16]) && card::is_spell_trap(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, tc.nil(), a.e) {
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
fn spell_trap(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_REMOVED as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    if !g.is_empty()
        && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_in_extra_m_zone(d, c, None)
        && duel::is_existing_matching_card(d, Some(&spelltrap_removable), a.tp, 0, LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        let rg = duel::get_matching_group(d, Some(&spelltrap_removable), a.tp, 0, LOCATION_GRAVE, NO_EX);
        if !rg.is_empty() {
            duel::break_effect(d);
            duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
fn spelltrap_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, c)
}
