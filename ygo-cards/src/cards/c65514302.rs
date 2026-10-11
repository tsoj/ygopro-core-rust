//! Magnet Bonding
use super::prelude::*;

pub const ID: u32 = 65514302;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[44839512],
    listed_series: &[SET_MAGNET_WARRIOR, SET_MAGNA_WARRIOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects (but you can only use each effect of "Magnet Bonding" once per turn)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.magnetthfilter(c)`
fn magnetthfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(44839512) or (c:IsLevelBelow(4) and c:IsSetCard(SET_MAGNET_WARRIOR))) and c:IsAbleToHand()
    (card::is_code(d, c, &[44839512]) || (card::is_level_below(d, c, 4) && card::is_set_card(d, c, &[SET_MAGNET_WARRIOR as u16]))) && card::is_able_to_hand(d, c, None)
}
/// `s.magnathfilter(c)`
fn magnathfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(8) and c:IsSetCard(SET_MAGNA_WARRIOR) and c:IsAbleToHand()
    card::get_level(d, c) == 8 && card::is_set_card(d, c, &[SET_MAGNA_WARRIOR as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.fextrafilter(c)`
fn fextrafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ROCK) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_race(d, c, RACE_ROCK) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(s.fextrafilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&fextrafilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    (Some(g), None)
}
/// `local fusion_params={fusfilter=..., matfilter=..., extrafil=s.fextra, extraop=Fusion.ShuffleMaterial}`
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_ROCK))),
        matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_race(d, c, RACE_ROCK) && card::is_able_to_deck(d, c))),
        extrafil: Some(std::sync::Arc::new(fextra)),
        extraop: Some(fusion::shuffle_material()),
        ..Default::default()
    }
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local b1=not Duel.HasFlagEffect(tp,id) and Duel.IsExistingMatchingCard(s.magnetthfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_matching_card(d, Some(&magnetthfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Duel.IsExistingMatchingCard(s.magnathfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && duel::is_existing_matching_card(d, Some(&magnathfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b3=not Duel.HasFlagEffect(tp,id+2) and Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b3 = !duel::has_flag_effect(d, tp, ID + 2, None) && {
        let mut a0 = a.clone();
        a0.chk = Some(0);
        fusion::summon_eff_tg_call(d, &fusion_params(), &a0)
    };
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    let op = op.map_or(0, |v| v as i64);
    effect::set_label(d, e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_TODECK)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_TODECK);
        // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.magnetthfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&magnetthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.magnathfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&magnathfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    } else if op == 3 {
        // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fusion_params(), a);
    }
}
