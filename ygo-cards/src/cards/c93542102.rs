//! Morphtronic Celfon
use super::prelude::*;

pub const ID: u32 = 93542102;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(cona)));
    effect::set_target(d, e1, Some(cost_target(tga)));
    effect::set_operation(d, e1, Some(operation(opa)));
    card::register_effect(d, c, e1, false);
    // confirm top deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(tgd)));
    effect::set_operation(d, e2, Some(operation(opd)));
    card::register_effect(d, c, e2, false);
}
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, h) && card::is_attack_pos(d, h)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, h) && card::is_defense_pos(d, h)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_MORPHTRONIC) and c:IsMonster()
    card::is_level_below(d, c, 4) && card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_monster(d, c)
}
fn tga(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0
            && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
fn tgd(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_MORPHTRONIC) and c:IsMonster()
    //   and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    cfilter(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn opa(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, tp, 1, 0)[0] as u32;
    // Duel.ConfirmDecktop(tp,dc)
    duel::confirm_decktop(d, tp, dc);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local g=Duel.GetDecktopGroup(tp,dc)
    let g = duel::get_decktop_group(d, tp, dc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:FilterSelect(tp,s.filter,1,1,nil,e,tp)
    let sg = group::filter_select(d, &g, tp, &|d, c| filter(d, c, a.e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon(d, &sg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
}
fn opd(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, tp, 1, 0)[0] as u32;
    // local g=Duel.GetDecktopGroup(tp,dc)
    let g = duel::get_decktop_group(d, tp, dc);
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, &g);
}
