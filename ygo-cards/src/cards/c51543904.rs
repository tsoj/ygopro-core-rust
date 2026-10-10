//! No.99 Hope Dragoon (Utopic Dragon)
use super::prelude::*;

pub const ID: u32 = 51543904;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RANK_UP_MAGIC, SET_NUMBER],
    xyz_number: Some(99),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon
    // Xyz.AddProcedure(c,nil,10,3,s.ovfilter,aux.Stringid(id,0),3,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(10),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Negate
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16]) && card::is_spell(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card_ctx(d, c, &[SET_UTOPIA as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.xyzop(e,tp,chk,mc)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    if a.int(2) == 0 {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND, 0, NO_EX);
    let cancellable = xyz::proc_cancellable(d);
    let tc = group::select_unselect(d, &g, Some(&CardSet::new()), tp, false, cancellable, None, None);
    if let Some(tc) = tc {
        duel::send_to_grave(d, tc, REASON_DISCARD | REASON_COST, None, None);
        true
    } else {
        false
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    let c = effect::get_handler(d, e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, false);
        }
    }
    duel::special_summon_complete(d);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) || !effect::is_monster_effect(d, re) {
        return false;
    }
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    tg.is_some_and(|tg| group::is_contains(&tg, c)) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) {
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
}
