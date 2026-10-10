//! Enchanting Fitting Room
use super::prelude::*;

pub const ID: u32 = 30531525;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsLevelBelow(3) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_NORMAL)
        && card::is_level_below(d, c, 3)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>3
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.ConfirmDecktop(tp,4)
    duel::confirm_decktop(d, a.tp, 4);
    // local g=Duel.GetDecktopGroup(tp,4):Filter(s.filter,nil,e,tp)
    let dg = duel::get_decktop_group(d, a.tp, 4);
    let mut g = group::filter(d, &dg, &|d, c| filter(d, c, a.e, a.tp), NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if ft > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if #g>0 then
    if !g.is_empty() {
        if ft <= 0 {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        } else if ft as usize >= g.len() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,ft,ft,nil)
            let sg = group::select(d, &g, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // g:Sub(sg)
            group::remove_card(&mut g, &sg);
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, a.tp);
}
