//! Number 46: Dragluon
use super::prelude::*;

pub const ID: u32 = 2978414;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(46), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 8 Dragon monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),8,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, sc, st, Some(tp)))), Some(8), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return not Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,0,1,e:GetHandler()) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e);
        !duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, 0, 1, h.into())
    })));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DRAGON) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.ctrlfilter(c)`
fn ctrlfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsFaceup() and c:IsControlerCanBeChanged()
    card::is_race(d, c, RACE_DRAGON) && card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.efftg(e,tp,...,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return e:GetLabel()==2 and chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.ctrlfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) == 2 && card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && ctrlfilter(d, chkc);
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.ctrlfilter,tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_target(d, Some(&ctrlfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b3=not Duel.HasFlagEffect(1-tp,id)
    let b3 = !duel::has_flag_effect(d, 1 - a.tp, ID, None);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    let labels: Vec<i64> = op.map(|x| x as i64).into_iter().collect();
    effect::set_label(d, a.e, &labels);
    // e:SetCategory(0)
    effect::set_category(d, a.e, 0);
    // e:SetProperty(0)
    effect::set_property(d, a.e, 0, 0);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, a.e, CATEGORY_CONTROL);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local g=Duel.SelectTarget(tp,s.ctrlfilter,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&ctrlfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,...)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --Special Summon 1 Dragon monster from your hand
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --Target 1 Dragon monster your opponent controls; take control of that target
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsRace(RACE_DRAGON) and tc:IsFaceup() then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_race(d, tc, RACE_DRAGON) && card::is_faceup(d, tc) {
            // Duel.GetControl(tc,tp)
            duel::get_control(d, tc, a.tp, 0, 0, None, None);
        }
    // elseif op==3 then
    } else if op == 3 {
        // local opp=1-tp
        let opp = 1 - a.tp;
        // if Duel.HasFlagEffect(opp,id) then return end
        if duel::has_flag_effect(d, opp, ID, None) {
            return;
        }
        // Duel.RegisterFlagEffect(opp,id,RESET_PHASE|PHASE_END|RESET_OPPO_TURN,0,1)
        duel::register_flag_effect(d, opp, ID, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 0, 1, 0);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // aux.RegisterClientHint(c,nil,tp,0,1,aux.Stringid(id,4),RESET_OPPO_TURN)
        aux::register_client_hint(d, c, 0, a.tp, 0, 1, Some(aux::stringid(ID, 4)), RESET_OPPO_TURN, None);
        // --Dragon monsters your opponent controls cannot activate their effects until the end of your opponent's turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e1, 0, LOCATION_MZONE);
        // e1:SetTarget(function(e,c) return c:IsRace(RACE_DRAGON) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_DRAGON))));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
