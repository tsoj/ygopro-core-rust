//! Dark Magician Girl the Dragon Knight
use super::prelude::*;

pub const ID: u32 = 43892408;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[1784686],
    material_setcode: &[SET_DARK_MAGICIAN, SET_MAGICIAN_GIRL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,false,false,CARD_DARK_MAGICIAN_GIRL,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
    fusion::add_proc_mix(
        d,
        c,
        false,
        false,
        vec![
            fusion::Mat::Code(CARD_DARK_MAGICIAN_GIRL),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
        ],
    );
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, lua_fn(splimit));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(cost)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(activate)));
    card::register_effect(d, c, e2, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let (se, st) = (a.e(1), a.int(3) as u64);
    (st & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64 || card::is_code(d, effect::get_handler(d, se).nil(), &[1784686])
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), 1, 1, REASON_COST, NO_EX);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
