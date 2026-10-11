//! Infernoble Knight Astolfo
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 95953557;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from the hand and change its level from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon monsters after banished
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1},EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 1, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.regtg)
    effect::set_target(d, e2, Some(cost_target(regtg)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToRemoveAsCost()
    //     and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.spcost(e,tp,...,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let rg = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, 0, h.into());
    // if chk==0 then return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0) end
    if a.chk == Some(0) {
        let rescon = aux::chkf_mmz(1);
        return !rg.is_empty() && matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(1), Some(1), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE)
    let rescon = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(1), Some(1), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, Value::Card(first.nil()));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LVCHANGE,c,1,tp,e:GetLabelObject():GetLevel())
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    let lv = card::get_level(d, lo) as i32;
    duel::set_possible_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(c)), 1, a.tp, lv);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lc=e:GetLabelObject()
    let lc = effect::get_label_object(d, a.e).as_card();
    // local lv=(lc and lc:HasLevel() and lc:IsLocation(LOCATION_REMOVED) and lc:IsFaceup()) and lc:GetLevel() or 0
    let lv: i64 = match lc {
        Some(lc) if card::has_level(d, lc) && card::is_location(d, lc, LOCATION_REMOVED) && card::is_faceup(d, lc) => card::get_level(d, lc) as i64,
        _ => 0,
    };
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 and lv>0
    //     and not c:IsLevel(lv) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0
        && lv > 0
        && !card::is_level(d, c, &[lv as u32])
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // --Make its level become the level of the banished monster
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.regtg(e,tp,...,chk)`
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_REMOVED as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=(Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(tp)) and 3 or 2
    let ct: u16 = if duel::is_phase(d, PHASE_STANDBY) && duel::is_turn_player(d, a.tp) { 3 } else { 2 };
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e1, LOCATION_REMOVED);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,ct)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, ct);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(lua_fn(|d, x: Args| duel::is_turn_player(d, x.p(1)))));
    // e1:SetOperation(s.spop2)
    effect::set_operation(d, e1, Some(lua_fn(spop2)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // c:SetTurnCounter(0)
    card::set_turn_counter(d, c, 0);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
/// `s.spop2(e,tp,...)`
fn spop2(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ct=c:GetTurnCounter()
    // ct=ct+1
    let ct = card::get_turn_counter(d, c) + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
    // if ct==2 then
    if ct == 2 {
        // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
        if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
            let nv = aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp));
            let g = duel::get_matching_group(d, Some(&nv), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
            if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
