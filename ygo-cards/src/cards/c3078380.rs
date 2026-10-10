//! Timaeus the United Dragon
use super::prelude::*;

pub const ID: u32 = 3078380;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Fusion Summon using materials from hand or field, including a Spellcaster monster
    // local params = {nil,nil,function(e,tp,mg) return nil,s.fcheck end}
    let p = fusion::SummonEff {
        extrafil: Some(std::sync::Arc::new(|_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet| (None, Some(lua_fn(fcheck))))),
        ..Default::default()
    };
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, p.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op = fusion::summon_eff_op(d, p);
    effect::set_operation(d, e2, Some(op));
    card::register_effect(d, c, e2, false);
}
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_able_to_grave_as_cost(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && ((card::is_monster(d, c) && card::is_race(d, c, RACE_SPELLCASTER)) || (card::is_spell_trap(d, c) && card::lists_code(d, c, &[CARD_DARK_MAGICIAN])))
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spcfilter(d, c, tp)), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spcfilter(d, c, tp)), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_race(d, c, RACE_SPELLCASTER), 1, NO_EX)
}
