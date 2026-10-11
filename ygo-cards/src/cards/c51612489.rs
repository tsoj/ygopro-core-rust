//! Riot's Reason
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 51612489;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 face-down Defense Position monster on the field to the hand, then the player who added it to their hand can Special Summon 1 monster from their hand in face-down Defense Position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    effect::set_operation(d, e1, Some(operation(rthop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // Change 1 face-down Defense Position monster on the field to Attack Position
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_POSITION);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(postg)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
}
/// `s.rthfilter(c)`
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE) and c:IsAbleToHand()
    card::is_position(d, c, POS_FACEDOWN_DEFENSE as u32) && card::is_able_to_hand(d, c, None)
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let att = duel::get_attacker(d).nil();
    card::is_controler(d, att, 1 - a.tp)
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(s.rthfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rthfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local tc=Duel.SelectTarget(tp,s.rthfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&rthfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tc:GetOwner(),LOCATION_HAND)
    let owner = card::get_owner(d, tc);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, owner, LOCATION_HAND as i32);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
    {
        // local sp_player=tc:GetControler()
        let sp_player = card::get_controler(d, tc);
        // Duel.ShuffleHand(sp_player)
        duel::shuffle_hand(d, sp_player);
        // if Duel.GetLocationCount(sp_player,LOCATION_MZONE,sp_player)>0
        //     and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,sp_player,LOCATION_HAND,0,1,nil,e,0,sp_player,false,false,POS_FACEDOWN_DEFENSE)
        //     and Duel.SelectYesNo(sp_player,aux.Stringid(id,2)) then
        if duel::get_location_count(d, sp_player, LOCATION_MZONE as u8, Some(sp_player), None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, sp_player, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
                sp_player,
                LOCATION_HAND,
                0,
                1,
                NO_EX,
            )
            && duel::select_yes_no(d, sp_player, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,sp_player,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, sp_player, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(sp_player,Card.IsCanBeSpecialSummoned,sp_player,LOCATION_HAND,0,1,1,nil,e,0,sp_player,false,false,POS_FACEDOWN_DEFENSE)
            let g = duel::select_matching_card(
                d,
                sp_player,
                Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, sp_player, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
                sp_player,
                LOCATION_HAND,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(g,0,sp_player,sp_player,false,false,POS_FACEDOWN_DEFENSE)
                duel::special_summon(d, g, 0, sp_player, sp_player, false, false, POS_FACEDOWN_DEFENSE as u8, None);
            }
        }
    }
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE) and c:IsCanChangePosition()
    card::is_position(d, c, POS_FACEDOWN_DEFENSE as u32) && card::is_can_change_position(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.ChangePosition(g,POS_FACEUP_ATTACK)
        duel::change_position(d, g, POS_FACEUP_ATTACK as u8, None, None, None, false);
    }
}
