//! Heosvarog the Mechanical Dawn
use super::prelude::*;

pub const ID: u32 = 8963089;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 LIGHT Machine monsters
    // Fusion.AddProcMixN(c,true,true,s.matfilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))), 2)]);
    // Add 1 card with an effect that Fusion Summons from your GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.regtg)
    effect::set_target(d, e1, Some(cost_target(regtg)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of an opponent's Spell/Trap or effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsSpellTrapEffect() and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(s.negcost)
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateActivation(ev) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::negate_activation(d, a.ev as u8);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT,fc,sumtype,tp) and c:IsRace(RACE_MACHINE,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(fc), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_MACHINE, Some(fc), sumtype, Some(tp))
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.regtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local turn_ct=Duel.GetTurnCount()
    let turn_ct = duel::get_turn_count(d, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function() return Duel.GetTurnCount()~=turn_ct end)
    effect::set_condition(d, e1, Some(condition(move |d, _a| duel::get_turn_count(d, None) != turn_ct)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESET_PHASE|PHASE_STANDBY,Duel.GetCurrentPhase()<=PHASE_STANDBY and 2 or 1)
    let count = if duel::get_current_phase(d) <= PHASE_STANDBY { 2 } else { 1 };
    effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, count);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsAbleToHand() then return end
    if !card::is_able_to_hand(d, c, None) {
        return false;
    }
    // local effs={c:GetOwnEffects()}
    // for _,eff in ipairs(effs) do if eff:IsHasCategory(CATEGORY_FUSION_SUMMON) then return true end end
    for eff in card::get_own_effects(d, c) {
        if effect::is_has_category(d, eff, CATEGORY_FUSION_SUMMON) {
            return true;
        }
    }
    // return false
    false
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, &g, true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    }
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsAbleToRemoveAsCost() and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_MACHINE) && card::is_able_to_remove_as_cost(d, c, None) && card::is_faceup(d, c)
}
/// `s.rescon(sg,e,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return (#sg==1 and sg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE))
    //   or sg:IsExists(Card.IsLocation,2,nil,LOCATION_GRAVE)
    let res = (sg.len() == 1 && group::is_exists(d, sg, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, NO_EX)) || group::is_exists(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 2, NO_EX);
    (res, false)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE,0,1,nil)
    //   or Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) || duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&costfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local cg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_REMOVE,s.rescon)
    let cg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), Some(&rescon), None, false).unwrap_or_default();
    // Duel.Remove(cg,POS_FACEUP,REASON_COST)
    duel::remove(d, &cg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsSpellTrapEffect() and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.ep != 1 - a.tp {
        return false;
    }
    if !effect::is_spell_trap_effect(d, a.re.nil()) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    duel::is_chain_negatable(d, a.ev as u8)
}
