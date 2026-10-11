//! Solfachord Happiness
use super::prelude::*;

pub const ID: u32 = 93481594;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.scale_of(c)`: `Card.GetScale` as a number for `GetClassCount` and `aux.dpcheck`
fn scale_i64(d: &mut Duel, c: CardId) -> i64 {
    card::get_scale(d, c) as i64
}
/// `s.efftg(e,tp,...,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler(),REASON_EFFECT)
    //     and Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil):GetClassCount(Card.GetScale)>=2
    let b1 = !duel::has_flag_effect(d, tp, ID + 1, None) && {
        let h = effect::get_handler(d, e);
        duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, h.into())
    } && {
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        group::get_class_count(d, &g, &scale_i64) >= 2
    };
    // local b2=not Duel.HasFlagEffect(tp,id+2) and Pendulum.PlayerCanGainAdditionalPendulumSummon(tp,id)
    let b2 = !duel::has_flag_effect(d, tp, ID + 2, None) && pendulum::player_can_gain_additional_pendulum_summon(d, tp, ID);
    // local b3=not Duel.HasFlagEffect(tp,id+3)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_PZONE,0,2,nil,e,tp)
    let b3 = !duel::has_flag_effect(d, tp, ID + 3, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_PZONE, 0, 2, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    // Duel.RegisterFlagEffect(tp,id+op,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + op as u32, RESET_PHASE | PHASE_END, 0, 1, 0);
    if op == 1 {
        // e:SetCategory(CATEGORY_HANDES+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_HANDES + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    } else if op == 2 {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_PZONE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_PZONE as i32);
    }
    true
}
/// `s.effop(e,tp,...)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    if op == 1 {
        // Discard 1 card, and if you do, add 2 "Solfachord" Pendulum Monsters with different Pendulum Scales from your Deck to your hand
        // if Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)==0 then return end
        if duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) == 0 {
            return;
        }
        // local dg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let dg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // local sdg=aux.SelectUnselectGroup(dg,e,tp,2,2,aux.dpcheck(Card.GetScale),1,tp,HINTMSG_ATOHAND)
        let dp = aux::dpcheck(scale_i64);
        let rs: aux::Rescon = &dp;
        let sdg = aux::select_unselect_group(d, &dg, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
        // if #sdg~=2 or Duel.SendtoHand(sdg,nil,REASON_EFFECT)~=2 then return end
        if sdg.len() != 2 || duel::send_to_hand(d, sdg.clone(), None, REASON_EFFECT, None) != 2 {
            return;
        }
        // Duel.ConfirmCards(1-tp,sdg:Match(Card.IsLocation,nil,LOCATION_HAND))
        let mut m = sdg.clone();
        group::match_(d, &mut m, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
        duel::confirm_cards(d, 1 - tp, m);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // if ct==0 then return end
        if ct == 0 {
            return;
        }
        // local hg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
        let hg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
        // if #hg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,4)) then return end
        if hg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
            return;
        }
        // ct=math.min(ct,Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)+1)
        let ct = ct.min(duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) + 1);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
        let ct = if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() { 1 } else { ct };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local shg=hg:Select(tp,1,ct,nil)
        let shg = group::select(d, &hg, tp, 1, ct as u16, false, NO_EX).nil();
        // if #shg>0 then
        if !shg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(shg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, shg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == 2 {
        // During your Main Phase this turn, you can conduct 1 Pendulum Summon of a "Solfachord" monster(s) in addition to your Pendulum Summon
        // Pendulum.GrantAdditionalPendulumSummon(e:GetHandler(),function(c) return c:IsSetCard(SET_SOLFACHORD) end,tp,LOCATION_HAND|LOCATION_EXTRA,aux.Stringid(id,5),aux.Stringid(id,6),id)
        let h = effect::get_handler(d, e).nil();
        let pf: pendulum::PendFilter = std::sync::Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8, _l: u32, _r: u32| card::is_set_card(d, x, &[SET_SOLFACHORD as u16]));
        pendulum::grant_additional_pendulum_summon(d, h, Some(pf), tp, LOCATION_HAND | LOCATION_EXTRA, Some(aux::stringid(ID, 5)), Some(aux::stringid(ID, 6)), ID, None, None);
    } else if op == 3 {
        // Special Summon 2 "Solfachord" cards from your Pendulum Zone
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return;
        }
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_PZONE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_PZONE, 0, NO_EX);
        // if #g==2 then
        if g.len() == 2 {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
