//! Arcana Force XXI - The World
use super::prelude::*;

pub const ID: u32 = 23846921;
pub const INFO: ScriptInfo = ScriptInfo { arcanareg: Some(arcanareg), toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin and apply the appropriate effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COIN);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(cointg)));
    effect::set_operation(d, e1, Some(operation(coinop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // s.arcanareg(c,Arcana.TossCoin(c,tp))
    let coin = aux::arcana::toss_coin(d, c, a.tp);
    arcanareg(d, c, coin);
}
fn arcanareg(d: &mut Duel, c: CardId, coin: i32) {
    // coin effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(skipcon)));
    effect::set_cost(d, e1, Some(cost_target(skipcost)));
    effect::set_target(d, e1, Some(cost_target(skiptg)));
    effect::set_operation(d, e1, Some(operation(skipop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_DRAW);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    aux::arcana::register_coin_result(d, c, coin);
}
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and Arcana.GetCoinResult(e:GetHandler())==COIN_HEADS
    if a.ep != a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    aux::arcana::get_coin_result(d, h) == Some(COIN_HEADS as i64)
}
fn skipcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_MZONE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,2,2,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_MZONE, 0, 2, 2, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn skiptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(1-tp,EFFECT_SKIP_TURN) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, 1 - a.tp, EFFECT_SKIP_TURN).is_none();
    }
    true
}
fn skipop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_SKIP_TURN);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Arcana.GetCoinResult(e:GetHandler())==COIN_TAILS
    if a.ep != 1 - a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    aux::arcana::get_coin_result(d, h) == Some(COIN_TAILS as i64)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=Duel.GetFieldCard(1-tp,LOCATION_GRAVE,Duel.GetFieldGroupCount(1-tp,LOCATION_GRAVE,0)-1)
    let n = duel::get_field_group_count(d, 1 - a.tp, LOCATION_GRAVE, 0) - 1;
    let tc = duel::get_field_card(d, 1 - a.tp, LOCATION_GRAVE, n as u32);
    // if tc then Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,0,0) end
    if let Some(tc) = tc {
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, 0, 0);
    }
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFieldCard(1-tp,LOCATION_GRAVE,Duel.GetFieldGroupCount(1-tp,LOCATION_GRAVE,0)-1)
    let n = duel::get_field_group_count(d, 1 - a.tp, LOCATION_GRAVE, 0) - 1;
    let tc = duel::get_field_card(d, 1 - a.tp, LOCATION_GRAVE, n as u32);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(tp,tc)
        duel::confirm_cards(d, a.tp, tc);
    }
}
