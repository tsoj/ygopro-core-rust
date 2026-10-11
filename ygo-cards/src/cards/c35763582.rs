//! Lunalight Silver Hound
use super::prelude::*;

pub const ID: u32 = 35763582;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LUNALIGHT], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Lunalight" monster from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Negate the activation of a Spell/Trap Card or effect activated on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_NEGATE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsReason(REASON_EFFECT) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sc = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sc);
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
            // You cannot Special Summon from the Extra Deck while you control that face-up monster, except "Lunalight" monsters
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            effect::set_range(d, e1, LOCATION_MZONE);
            effect::set_absolute_range(d, e1, tp, 1, 0);
            effect::set_condition(d, e1, Some(condition(cond2)));
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_set_card(d, c, &[SET_LUNALIGHT as u16]))));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, sc, e1, true);
        }
    }
}
/// `function(e) return e:GetHandler():IsControler(e:GetOwnerPlayer()) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let owner = effect::get_owner_player(d, a.e);
    card::is_controler(d, h, owner)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_spell_trap_effect(d, re)
        && (duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() & LOCATION_ONFIELD as i64) > 0
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.negcostfilter(c)`
fn negcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_type(d, c, TYPE_FUSION) && card::is_able_to_remove_as_cost(d, c, None)
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&negcostfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let mut g = duel::select_matching_card(d, tp, Some(&negcostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    duel::negate_activation(d, a.ev as u8);
}
