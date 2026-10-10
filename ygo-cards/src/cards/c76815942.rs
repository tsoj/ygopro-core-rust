//! LL - Independent Nightingale
use super::prelude::*;

pub const ID: u32 = 76815942;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_LYRILUSC],
    listed_names: &[8491961],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,48608796,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_LYRILUSC))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(48608796),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_LYRILUSC as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // increase level
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e1, Some(condition(lvcon)));
    effect::set_operation(d, e1, Some(operation(lvop)));
    card::register_effect(d, c, e1, false);
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    effect::set_label_object(d, e0, Value::Effect(e1));
    card::register_effect(d, c, e0, false);
    // immune
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, lua_fn(efilter));
    card::register_effect(d, c, e2, false);
    // atk up
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    card::register_effect(d, c, e3, false);
    // damage
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_target(d, e4, Some(cost_target(damtg)));
    effect::set_operation(d, e4, Some(operation(damop)));
    card::register_effect(d, c, e4, false);
}
/// `s.matval(c,sc)`
fn matval(d: &mut Duel, c: CardId, sc: CardId) -> i64 {
    let b = card::is_original_set_card(d, c, &[SET_LYRILUSC as u16]);
    if (b || card::is_original_code(d, c, &[8491961])) && card::is_type_ctx(d, c, TYPE_XYZ, Some(sc), SUMMON_TYPE_FUSION as u64, None) {
        return card::get_overlay_count(d, c) as i64;
    }
    0
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let mat = card::get_material(d, c);
    let val = group::get_sum(d, &mat, &|d, mc| matval(d, mc, c));
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[val]);
    0
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c) && effect::get_label(d, a.e) > 0
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, effect::get_label(d, e));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_level(d, c) as i64 * 500
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - tp);
    let c = effect::get_handler(d, a.e).nil();
    let amount = card::get_level(d, c) as i64 * 500;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, amount as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) {
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
        let p = info[0].int() as u8;
        let amount = card::get_level(d, c) as i64 * 500;
        duel::damage(d, p, amount, REASON_EFFECT, false, None);
    }
}
