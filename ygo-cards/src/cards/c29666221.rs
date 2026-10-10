//! Orcustrated Attack
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 29666221;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ORCUST as u32, SET_WORLD_LEGACY as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.spcheck(sg,tp,exg,dg)`
fn spcheck(d: &mut Duel, sg: &CardSet, dg: &CardSet) -> (bool, bool) {
    (group::is_exists(d, dg, &|_d, _c| true, 1, sg.into()), false)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ORCUST as u16, SET_WORLD_LEGACY as u16])
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    let dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e)), tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        if effect::get_label(d, e) == 1 {
            effect::set_label(d, e, &[0]);
            return duel::check_release_group_cost(
                d,
                tp,
                Some(&cfilter),
                1,
                1,
                false,
                Some(&|d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet| spcheck(d, sg, &dg)),
                NO_EX,
            );
        } else {
            return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
        }
    }
    if effect::get_label(d, e) == 1 {
        effect::set_label(d, e, &[0]);
        let sg = duel::select_release_group_cost(
            d,
            tp,
            Some(&cfilter),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet| spcheck(d, sg, &dg)),
            NO_EX,
        );
        duel::release(d, sg, REASON_COST, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
