//! Number 60: Dugares the Timeless
use super::prelude::*;

pub const ID: u32 = 66011101;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(60), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsPlayerCanDraw(tp,2)
    let b1 = duel::is_player_can_draw(d, a.tp, 2);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    let b3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        a.tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    )
    .nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
        effect::set_category(d, a.e, CATEGORY_DRAW + CATEGORY_HANDES);
        // e:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_PLAYER_TARGET, 0);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, a.tp);
        // Duel.SetTargetParam(2)
        duel::set_target_param(d, 2);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local skipped_phase=(op==1 and PHASE_DRAW) or (op==2 and PHASE_MAIN1) or (op==3 and PHASE_BATTLE)
    let skipped_phase: u32 = if op == 1 {
        PHASE_DRAW
    } else if op == 2 {
        PHASE_MAIN1
    } else {
        PHASE_BATTLE
    };
    // local skip_effect_code=(op==1 and EFFECT_SKIP_DP) or (op==2 and EFFECT_SKIP_M1) or (op==3 and EFFECT_SKIP_BP)
    let skip_effect_code: u32 = if op == 1 {
        EFFECT_SKIP_DP
    } else if op == 2 {
        EFFECT_SKIP_M1
    } else {
        EFFECT_SKIP_BP
    };
    // local turn_ct=Duel.GetTurnCount()
    let turn_ct = duel::get_turn_count(d, None);
    // local curr_phase=Duel.GetCurrentPhase()
    let curr_phase = duel::get_current_phase(d);
    // local skip_next_turn=(Duel.IsTurnPlayer(tp) and curr_phase>=skipped_phase) or skipped_phase==PHASE_BATTLE
    let skip_next_turn = (duel::is_turn_player(d, a.tp) && curr_phase >= skipped_phase) || skipped_phase == PHASE_BATTLE;
    // local reset_ct=skip_next_turn and 2 or 1
    let reset_ct: u16 = if skip_next_turn { 2 } else { 1 };
    // Skip your next Draw Phase/your next Main Phase 1/the Battle Phase of your next turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3+op))
    effect::set_description(d, e1, aux::stringid(ID, (3 + op) as u32));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(skip_effect_code)
    effect::set_code(d, e1, skip_effect_code);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // if skip_next_turn then
    if skip_next_turn {
        // e1:SetCondition(function() return Duel.GetTurnCount()~=turn_ct end)
        effect::set_condition(d, e1, Some(condition(move |d, _a| duel::get_turn_count(d, None) != turn_ct)));
    }
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,reset_ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, reset_ct);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    if op == 1 {
        // Draw 2 cards, then discard 1 card
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        let p = info[0].int() as u8;
        let cnt = info[1].int() as u32;
        // if Duel.Draw(p,d,REASON_EFFECT)==2 then
        if duel::draw(d, p, cnt, REASON_EFFECT) == 2 {
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
            duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    } else if op == 2 {
        // Special Summon 1 monster from your GY in Defense Position
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    } else if op == 3 {
        // Double the ATK of 1 monster you control until the end of this turn
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
        // local tc=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.HintSelection(tc,true)
            duel::hint_selection(d, tc, true);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
            // e2:SetValue(tc:GetAttack()*2)
            let atk = card::get_attack(d, tc) as i64 * 2;
            effect::set_value(d, e2, atk);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
