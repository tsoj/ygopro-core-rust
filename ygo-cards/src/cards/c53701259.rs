//! Awakening of the Sacred Beasts
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 53701259;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[69890967, 6007213, 32491822], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Gain LP (extra handling from Ghost Sister & Spooky Dogwood 儚無みずき)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(reccon1)));
    effect::set_operation(d, e2, Some(operation(recop1)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_condition(d, e4, Some(condition(regcon)));
    effect::set_operation(d, e4, Some(operation(regop)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e5, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_condition(d, e5, Some(condition(reccon2)));
    effect::set_operation(d, e5, Some(operation(recop2)));
    card::register_effect(d, c, e5, false);
    effect::set_label_object(d, e4, Value::Effect(e5));
    // Negate effect
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e6, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e6, LOCATION_SZONE);
    effect::set_condition(d, e6, Some(condition(discon)));
    effect::set_operation(d, e6, Some(operation(disop)));
    card::register_effect(d, c, e6, false);
    // Banish
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    effect::set_property(d, e7, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e7, EFFECT_TO_GRAVE_REDIRECT);
    effect::set_range(d, e7, LOCATION_SZONE);
    effect::set_target_range(d, e7, 0, 0xff);
    effect::set_value(d, e7, LOCATION_REMOVED as i64);
    effect::set_condition(d, e7, Some(condition(rmcon)));
    effect::set_target(d, e7, Some(effect_card_fn(rmtg)));
    card::register_effect(d, c, e7, false);
    // Return to hand
    let e8 = effect::create_effect(d, c);
    effect::set_description(d, e8, aux::stringid(ID, 0));
    effect::set_category(d, e8, CATEGORY_TOHAND);
    effect::set_type(d, e8, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e8, EVENT_FREE_CHAIN);
    effect::set_range(d, e8, LOCATION_SZONE);
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    effect::set_cost(d, e8, Some(condition(thcon)));
    effect::set_target(d, e8, Some(cost_target(thtg)));
    effect::set_operation(d, e8, Some(operation(thop)));
    card::register_effect(d, c, e8, false);
}
/// `function(c) return c:IsFaceup() and c:IsCode(69890967,6007213,32491822) end`
fn sbfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[69890967, 6007213, 32491822])
}
/// `Duel.GetMatchingGroup(s.sbfilter,tp,LOCATION_ONFIELD,0,nil):GetClassCount(Card.GetCode)`
fn sbcount(d: &mut Duel, tp: u8) -> usize {
    let g = duel::get_matching_group(d, Some(&sbfilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64)
}
/// `function(c,sp) ... end`
fn filter(d: &mut Duel, c: CardId, sp: u8) -> bool {
    if card::is_location(d, c, LOCATION_MZONE) {
        card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::get_summon_player(d, c) == sp
    } else {
        card::is_previous_position(d, c, POS_FACEUP as u32) && card::is_previous_location(d, c, LOCATION_MZONE) && card::get_summon_player(d, c) == sp
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(s.filter,1,nil,1-tp) and s.sbcount(tp)>0 and (not re or (not re:IsHasType(EFFECT_TYPE_ACTIONS) or re:IsHasType(EFFECT_TYPE_CONTINUOUS))) end`
fn reccon1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - tp), 1, NO_EX)
        && sbcount(d, tp) > 0
        && match a.re {
            None => true,
            Some(re) => !effect::is_has_type(d, re, EFFECT_TYPE_ACTIONS) || effect::is_has_type(d, re, EFFECT_TYPE_CONTINUOUS),
        }
}
/// `function(c) if c:IsLocation(LOCATION_MZONE) then return c:GetAttack() else return c:GetPreviousAttackOnField() end end`
fn sum(d: &mut Duel, c: CardId) -> i64 {
    if card::is_location(d, c, LOCATION_MZONE) {
        card::get_attack(d, c) as i64
    } else {
        card::get_previous_attack_on_field(d, c) as i64
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) local g=eg:Filter(s.filter,nil,1-tp) ... end`
fn recop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, 1 - tp), NO_EX);
    if group::get_count(&g) > 0 {
        let total = group::get_sum(d, &g, &|d, c| sum(d, c));
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        if duel::recover(d, tp, total, REASON_EFFECT, false, None) != 0 {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(s.filter,1,nil,1-tp) and s.sbcount(tp)>0 and re:IsHasType(EFFECT_TYPE_ACTIONS) and not re:IsHasType(EFFECT_TYPE_CONTINUOUS) end`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let re = a.re.nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - tp), 1, NO_EX)
        && sbcount(d, tp) > 0
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIONS)
        && !effect::is_has_type(d, re, EFFECT_TYPE_CONTINUOUS)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) local g=eg:Filter(s.filter,nil,1-tp) ... end`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, 1 - tp), NO_EX);
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
    let e5 = effect::get_label_object(d, a.e).as_effect().nil();
    let add = group::get_sum(d, &g, &|d, c| sum(d, c));
    let old = effect::get_label(d, e5);
    effect::set_label(d, e5, &[add + old]);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return Duel.GetFlagEffect(tp,id)>0 end`
fn reccon2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, a.tp, ID) > 0
}
/// `function(e,tp,eg,ep,ev,re,r,rp) ... end`
fn recop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::reset_flag_effect(d, tp, ID);
    let rec = effect::get_label(d, a.e);
    effect::set_label(d, a.e, &[0]);
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    if duel::recover(d, tp, rec, REASON_EFFECT, false, None) != 0 {
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return re:IsMonsterEffect() and rp==1-tp and s.sbcount(tp)>1 and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE end`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    effect::is_monster_effect(d, re)
        && a.rp == 1 - tp
        && sbcount(d, tp) > 1
        && duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() == LOCATION_MZONE as i64
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return s.sbcount(e:GetHandlerPlayer())>2 end`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    sbcount(d, p) > 2
}
/// `function(e,c) return c:IsMonster() and c:GetOwner()~=e:GetHandlerPlayer() and Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),c) end`
fn rmtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    card::is_monster(d, c) && card::get_owner(d, c) != p && duel::is_player_can_remove(d, p, Some((c, None)))
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(tp) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevel,10),tp,LOCATION_MZONE,0,1,nil) end`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_turn_player(d, tp) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_level(d, c, &[10])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_trap(d, c) && card::is_type(d, c, TYPE_CONTINUOUS) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX) && card::get_flag_effect(d, c, ID) == 0;
    }
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if group::get_count(&g) > 0 {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
