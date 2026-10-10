//! Dogmatika Punishment
use super::prelude::*;

pub const ID: u32 = 82956214;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 monster with an equal or higher ATK from your Extra Deck to the GY and destroy the targeted monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_EXTRA,0,1,nil,c:GetAttack())
    if !card::is_faceup(d, c) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, atk)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.tgfilter(c,atk)`
fn tgfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsAttackAbove(atk) and c:IsAbleToGrave()
    card::is_attack_above(d, c, atk) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.desfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && desfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_EXTRA,0,1,1,nil,tc:GetAttack()):GetFirst()
        let atk = card::get_attack(d, tc);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| tgfilter(d, c, atk)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) then
        if let Some(sc) = sc {
            if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0 && card::is_location(d, sc, LOCATION_GRAVE) {
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local ct=Duel.IsTurnPlayer(tp) and 2 or 1
    let ct: u16 = if duel::is_turn_player(d, a.tp) { 2 } else { 1 };
    // Until the end of your next turn after this card resolves, you cannot Special Summon monsters from the Extra Deck
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, ct);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
