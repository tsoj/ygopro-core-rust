//! Prank-Kids Rocket Ride
use super::prelude::*;

pub const ID: u32 = 18514525;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PRANK_KIDS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion summon procedure
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PRANK_KIDS),2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_PRANK_KIDS as u16], Some(fc), st, Some(tp)))), 2)]);
    // If fusion summoned, lose 1000 ATK, also can attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon non-fusion "Prank-Kids" monsters with different names from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=c:GetAttack()
    let atk = card::get_attack(d, c);
    // if not c:IsRelateToEffect(e) or atk<1000 then return end
    if !card::is_relate_to_effect(d, c, a.e) || atk < 1000 {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(-1000)
    effect::set_value(d, e1, -1000);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if c:GetAttack()-atk==-1000 then
    if card::get_attack(d, c) - atk == -1000 {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3205)
        effect::set_description(d, e2, 3205);
        // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_PRANK_KIDS) and not c:IsType(TYPE_FUSION)
    //   and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_PRANK_KIDS as u16])
        && !card::is_type(d, c, TYPE_FUSION)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcheck(sg,e,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==2
    (group::get_class_count(d, sg, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) == 2, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,0) end
    if a.chk == Some(0) {
        return ft > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&spcheck), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&spcheck), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,#tg,0,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), n, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<#g or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    let loc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if loc < g.len() as i32 || (g.len() > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
