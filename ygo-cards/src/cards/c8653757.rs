//! Photon Timestop
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 8653757;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_GALAXYEYES_P_DRAGON],
    listed_series: &[SET_PHOTON, SET_GALAXY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // End turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.skipcon)
    effect::set_condition(d, e2, Some(condition(skipcon)));
    // e2:SetTarget(s.skiptg)
    effect::set_target(d, e2, Some(cost_target(skiptg)));
    // e2:SetOperation(s.skipop)
    effect::set_operation(d, e2, Some(operation(skipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,ft,tp)
fn filter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return c:IsSetCard({SET_PHOTON,SET_GALAXY}) and c:IsContinuousSpellTrap()
    //     and (c:IsAbleToHand() or (ft>0 and not c:IsForbidden() and c:CheckUniqueOnField(tp)))
    if !(card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16]) && card::is_continuous_spell_trap(d, c)) {
        return false;
    }
    card::is_able_to_hand(d, c, None) || (ft > 0 && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,ft,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ft, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,ft,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, ft, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    // aux.ToHandOrElse(tc,tp,
    //     function(c)
    //         return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    //     end,
    //     function(c)
    //         Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
    //     end,
    //     aux.Stringid(id,3)
    // )
    aux::to_hand_or_else(
        d,
        tc,
        tp,
        Some(&|d, c| {
            duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
                && !card::is_forbidden(d, c)
                && card::check_unique_on_field(d, c, tp, None, None)
        }),
        Some(&|d, g: &CardSet| {
            for c in group::iter(d, g) {
                duel::move_to_field(d, c, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
            }
            0
        }),
        Some(aux::stringid(ID, 3)),
    );
}
// function s.skipcon(e,tp,eg,ep,ev,re,r,rp)
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return re and rp==1-tp and Duel.IsTurnPlayer(1-tp) and c:IsPreviousPosition(POS_FACEDOWN)
    //     and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD)
    a.re.is_some()
        && a.rp == 1 - a.tp
        && duel::is_turn_player(d, 1 - a.tp)
        && card::is_previous_position(d, c, POS_FACEDOWN)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:GetOverlayGroup():IsExists(Card.IsCode,1,nil,CARD_GALAXYEYES_P_DRAGON)
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[CARD_GALAXYEYES_P_DRAGON]), 1, NO_EX)
}
// function s.skiptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn skiptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_GALAXYEYES_P_DRAGON),tp,LOCATION_ONFIELD,0,1,nil)
    //     or Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON]))),
            a.tp,
            LOCATION_ONFIELD,
            0,
            1,
            NO_EX,
        ) || duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
// function s.skipop(e,tp,eg,ep,ev,re,r,rp)
fn skipop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.SkipPhase(1-tp,PHASE_DRAW,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_DRAW, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(1-tp,PHASE_STANDBY,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_STANDBY, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(1-tp,PHASE_MAIN1,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_MAIN1, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1,1)
    duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
    // Duel.SkipPhase(1-tp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
