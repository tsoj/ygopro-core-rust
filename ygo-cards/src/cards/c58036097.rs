//! Melffy Playtime Surprise
use super::prelude::*;

pub const ID: u32 = 58036097;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MELFFY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.immcon)
    effect::set_condition(d, e2, Some(condition(immcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.immop)
    effect::set_operation(d, e2, Some(operation(immop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE as u32));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `fusion_params={handler=c,fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_BEAST)}`
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_BEAST as u64))),
        ..Default::default()
    }
}
/// The Lua `(e,tp,eg,ep,ev,re,r,rp,chk)` arguments with another `chk`.
fn with_chk(a: &EffArgs, chk: Option<i64>) -> EffArgs {
    EffArgs {
        e: a.e,
        tp: a.tp,
        eg: a.eg.clone(),
        ep: a.ep,
        ev: a.ev,
        re: a.re,
        r: a.r,
        rp: a.rp,
        chk,
        extra: a.extra.clone(),
    }
}
/// `s.rthfilter(c)`
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MELFFY) and c:IsMonster() and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.rthfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && rthfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rthfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rthfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.rthfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&rthfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_FUSION_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_FUSION_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
    {
        // Duel.ShuffleHand(tc:GetControler())
        let p = card::get_controler(d, tc);
        duel::shuffle_hand(d, p);
        // if Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        let fp = fusion_params();
        if fusion::summon_eff_tg_call(d, &fp, &with_chk(a, Some(0))) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
            fusion::summon_eff_op_call(d, &fp, &with_chk(a, None));
        }
    }
}
/// `s.immconfilter(c)`
fn immconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MELFFY) and c:IsXyzMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_xyz_monster(d, c) && card::is_faceup(d, c)
}
/// `s.immcon(e,tp,...)`
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.immconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&immconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.immop(e,tp,...)`
fn immop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_MELFFY) and c:IsSummonLocation(LOCATION_EXTRA) and c:IsFaceup() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| {
        card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c)
    })));
    // e1:SetValue(function(e,re) return re:GetHandlerPlayer()==1-e:GetHandlerPlayer() and re:IsActivated() end)
    effect::set_value_fn(d, e1, lua_fn(immop_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(e,re) return re:GetHandlerPlayer()==1-e:GetHandlerPlayer() and re:IsActivated() end`
fn immop_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    effect::get_handler_player(d, re) == 1 - effect::get_handler_player(d, e) && effect::is_activated(d, re)
}
