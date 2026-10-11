//! Cyberse Magician
use super::prelude::*;

pub const ID: u32 = 24731391;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[34767865],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Halve damage
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value_fn(d, e1, lua_fn(val));
    card::register_effect(d, c, e1, false);
    // target
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_condition(d, e3, Some(condition(atcon)));
    effect::set_value_fn(d, e3, effect_card_fn(atlimit));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    effect::set_condition(d, e4, Some(condition(atcon)));
    effect::set_target(d, e4, Some(effect_card_fn(tglimit)));
    effect::set_value_fn(d, e4, aux::tgoval());
    card::register_effect(d, c, e4, false);
    // atk up
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e5, Some(condition(atkcon)));
    effect::set_value(d, e5, 1000);
    card::register_effect(d, c, e5, false);
    // add to hand
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 1));
    effect::set_category(d, e6, CATEGORY_TOHAND);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e6, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e6, EVENT_DESTROYED);
    effect::set_condition(d, e6, Some(condition(thcon)));
    effect::set_target(d, e6, Some(cost_target(thtg)));
    effect::set_operation(d, e6, Some(operation(thop)));
    card::register_effect(d, c, e6, false);
}
/// `s.val(e,re,dam,r,rp,rc)`: `math.floor(dam/2)`
fn val(_d: &mut Duel, a: Args) -> i64 {
    a.int(2).div_euclid(2)
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsLinkMonster,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_link_monster(d, c)), effect::get_handler_player(d, a.e), LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atlimit(e,c)`: `c~=e:GetHandler()`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_handler(d, e) != Some(c)
}
/// `s.tglimit(e,c)`: `c~=e:GetHandler()`
fn tglimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_handler(d, e) != Some(c)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and bc and bc:IsLinkMonster()
    duel::is_phase(d, PHASE_DAMAGE_CAL) && bc.is_some_and(|bc| card::is_link_monster(d, bc))
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsMonster() and c:IsAbleToHand()
    card::is_race(d, c, RACE_CYBERSE) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_EFFECT) and rp==1-tp and c:IsPreviousControler(tp)
    card::is_reason(d, c, REASON_EFFECT) && a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
