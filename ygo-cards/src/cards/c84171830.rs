//! Domain of the True Monarchs
use super::prelude::*;

pub const ID: u32 = 84171830;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(effect_card_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetOperation(s.effop2)
    effect::set_operation(d, e3, Some(operation(effop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atk up
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(effect_card_fn(atktg)));
    // e4:SetValue(800)
    effect::set_value(d, e4, 800);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // lv change
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.lvtg)
    effect::set_target(d, e5, Some(cost_target(lvtg)));
    // e5:SetOperation(s.lvop)
    effect::set_operation(d, e5, Some(operation(lvop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // clock lizard
    // local e6=aux.createContinuousLizardCheck(c,LOCATION_FZONE,nil,0xff,0xff)
    let e6 = aux::create_continuous_lizard_check(d, c, LOCATION_FZONE, None, Some(0xff), Some(0xff));
    // e6:SetCondition(s.discon)
    effect::set_condition(d, e6, Some(condition(discon)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if e:GetHandler():GetFlagEffect(id)>0 then e:GetHandler():ResetFlagEffect(id) return false end
    let h = effect::get_handler(d, e).nil();
    if card::get_flag_effect(d, h, ID) > 0 {
        card::reset_flag_effect(d, h, ID);
        return false;
    }
    // return c:IsLocation(LOCATION_EXTRA)
    card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTributeSummoned()
    card::is_tribute_summoned(d, c)
}
/// `s.discon(e)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,0)==0
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(s.cfilter,tp,0,LOCATION_MZONE,1,nil)
    duel::get_field_group_count(d, tp, LOCATION_EXTRA, 0) == 0
        && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&cfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and d and d:IsControler(1-tp)
    if !duel::is_phase(d, PHASE_DAMAGE_CAL) {
        return false;
    }
    match dd {
        None => false,
        Some(dd) => card::is_controler(d, dd, 1 - tp),
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c==Duel.GetAttacker() and c:IsTributeSummoned()
    duel::get_attacker(d) == Some(c) && card::is_tribute_summoned(d, c)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetAttack()==2800 and c:GetDefense()==1000
    card::get_attack(d, c) == 2800 && card::get_defense(d, c) == 1000
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(-2)
        effect::set_value(d, e1, -2);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 1);
        // g:GetFirst():RegisterEffect(e1)
        let tc = group::get_first(d, &g).nil();
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(function(c) return c:IsTributeSummoned() and c:IsReason(REASON_FUSION) end,1,nil)
    match a.eg.clone() {
        None => false,
        Some(eg) => group::is_exists(
            d,
            &eg,
            &|d, c| card::is_tribute_summoned(d, c) && card::is_reason(d, c, REASON_FUSION),
            1,
            NO_EX,
        ),
    }
}
/// `s.effop2(e,tp,eg,ep,ev,re,r,rp)`
fn effop2(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,0,0,0)
    let h = effect::get_handler(d, a.e).nil();
    let _ = card::register_flag_effect(d, h, ID, 0, 0, 0, 0, 0);
}
