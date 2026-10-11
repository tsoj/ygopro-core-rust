//! Lev Shaddoll Fusion
//! 凍てし心が映す神影
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 34950192;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: Send 1 Fusion Monster from your Extra Deck to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Shaddoll" Fusion Monster with a different Attribute from your Extra Deck (this is treated as a Fusion Summon), but change its ATK to 0
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.AND(s.spcost,s.effcost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost_target(spcost), cost_target(effcost)])));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not c:IsSummonLocation(LOCATION_EXTRA) or c:IsSetCard(SET_SHADDOLL) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(activity_counter));
}
// function(c) return not c:IsSummonLocation(LOCATION_EXTRA) or c:IsSetCard(SET_SHADDOLL) end
fn activity_counter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA) || card::is_set_card(d, c, &[SET_SHADDOLL as u16])
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSetCard(SET_SHADDOLL) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_set_card(d, c, &[SET_SHADDOLL as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFusionMonster() and c:IsAbleToGrave()
    card::is_fusion_monster(d, c) && card::is_able_to_grave(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
// function s.spcostfilter(c,e,tp)
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFusionMonster() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c:GetAttribute(),c)
    if !card::is_fusion_monster(d, c) {
        return false;
    }
    let attr = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, attr, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,attr,cost_c)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, attr: u32, cost_c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsFusionMonster() and c:IsAttributeExcept(attr)
    //     and c:CheckFusionMaterial() and Duel.GetLocationCountFromEx(tp,tp,cost_c,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false)
    card::is_set_card(d, c, &[SET_SHADDOLL as u16])
        && card::is_fusion_monster(d, c)
        && card::is_attribute_except(d, c, attr, None, 0, None)
        && card::check_fusion_material(d, c, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), cost_c.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION as u32, tp, false, false, None, None, None)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // local sc=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let g = aux::select_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    let sc = group::get_first(d, &g).nil();
    // e:SetLabel(sc:GetAttribute())
    let attr = card::get_attribute(d, sc);
    effect::set_label(d, e, &[attr as i64]);
    // Duel.Release(sc,REASON_COST)
    duel::release(d, sc, REASON_COST, None);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetParam(e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_target_param(d, label as u32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local attr=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let attr = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,attr):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter_op(d, c, e, tp, attr)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummonStep(sc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, sc, SUMMON_TYPE_FUSION as u32, tp, tp, false, false, POS_FACEUP as u8, None) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
    // if Duel.SpecialSummonComplete()>0 then
    if duel::special_summon_complete(d) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
// the spfilter of s.spop: (c, e, tp, attr) with the cost card absent
fn spfilter_op(d: &mut Duel, c: CardId, e: EffectId, tp: u8, attr: u32) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsFusionMonster() and c:IsAttributeExcept(attr)
    //     and c:CheckFusionMaterial() and Duel.GetLocationCountFromEx(tp,tp,cost_c,c)>0 ...
    // (cost_c is nil here: the summon is not a cost)
    card::is_set_card(d, c, &[SET_SHADDOLL as u16])
        && card::is_fusion_monster(d, c)
        && card::is_attribute_except(d, c, attr, None, 0, None)
        && card::check_fusion_material(d, c, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION as u32, tp, false, false, None, None, None)
}
