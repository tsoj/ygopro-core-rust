//! SPYRAL GEAR - Fully Armed
use super::prelude::*;

pub const ID: u32 = 73828446;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41091257], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_code(d, c, &[41091257]))), None, None, None, None, None, None);
    // atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 1000);
    card::register_effect(d, c, e2, false);
    // banish
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_REMOVE);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_condition(d, e4, Some(condition(rmcon)));
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    effect::set_operation(d, e4, Some(operation(rmop)));
    card::register_effect(d, c, e4, false);
    // special summon
    let e5 = effect::create_effect(d, c);
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    effect::set_condition(d, e5, Some(condition(spcon)));
    effect::set_target(d, e5, Some(cost_target(sptg)));
    effect::set_operation(d, e5, Some(operation(spop)));
    card::register_effect(d, c, e5, false);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg);
    let h = effect::get_handler(d, a.e).nil();
    first == card::get_equip_target(d, h)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let bc_opt = card::get_battle_target(d, group::get_first(d, &eg).nil());
    // e:SetLabelObject(bc)
    effect::set_label_object(d, e, bc_opt.map_or(Value::Nil, Value::Card));
    if a.chk == Some(0) {
        let bc = bc_opt.nil();
        return card::is_monster(d, bc)
            && !card::is_type(d, bc, TYPE_TOKEN)
            && card::is_able_to_remove(d, bc, None, None, None)
            && !card::is_location(d, bc, LOCATION_REMOVED)
            && card::is_public(d, bc)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, bc.into());
    }
    let bc = bc_opt.nil();
    duel::set_target_card(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(bc)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_able_to_remove(d, bc, None, None, None)
        && !card::is_location(d, bc, LOCATION_REMOVED)
        && card::is_public(d, bc)
        && card::is_monster(d, bc)
        && !card::is_type(d, bc, TYPE_TOKEN)
        && card::is_relate_to_effect(d, bc, a.e)
    {
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, bc.into());
        if g.is_empty() {
            return;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let mut sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::hint_selection(d, sg.clone(), true);
        group::add_card(&mut sg, &CardSet::from_card(bc));
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY) && card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEUP)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[41091257]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
