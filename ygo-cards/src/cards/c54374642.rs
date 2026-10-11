//! A.I.'s Show
use super::prelude::*;

pub const ID: u32 = 54374642;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_IGNISTER as u16]) && card::is_link_above(d, c, 3) && card::is_faceup(d, c) && card::get_sequence(d, c) > 4
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    let atk = duel::get_attacker(d).nil();
    card::is_controler(d, atk, 1 - a.tp) && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_CYBERSE)
        && !card::is_type(d, c, TYPE_LINK)
        && card::get_attack(d, c) == 2300
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetSum(Card.GetAttack)<=e:GetLabel()
    let sum = group::get_sum(d, sg, &|d, c| card::get_attack(d, c) as i64);
    let label = effect::get_label(d, e.nil());
    (sum <= label, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d).nil();
    if a.chk == Some(0) {
        return card::is_on_field(d, tg)
            && card::is_controler(d, tg, 1 - a.tp)
            && card::is_attack_above(d, tg, 2300)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_battle(d, tc) && card::is_attack_above(d, tc, 2300) {
            // local tg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
            let tg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
            if tg.is_empty() {
                return;
            }
            // e:SetLabel(tc:GetAttack())
            let atk = card::get_attack(d, tc);
            effect::set_label(d, a.e, &[atk as i64]);
            let maxc = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ), None).0;
            let rs: aux::Rescon = &rescon;
            let sg = aux::select_unselect_group(d, &tg, Some(a.e), tp, Some(1), Some(maxc as usize), Some(rs), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
            // e:SetLabel(0)
            effect::set_label(d, a.e, &[0]);
            if duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                duel::break_effect(d);
                duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
            }
        }
    }
}
