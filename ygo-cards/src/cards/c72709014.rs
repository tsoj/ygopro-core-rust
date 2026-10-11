//! Trial of the Princesses
use super::prelude::*;

pub const ID: u32 = 72709014;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75917088, 2316186, 46128076], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,81383947,46128076))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_code(d, x, &[81383947, 46128076]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(800)
    effect::set_value(d, e2, 800);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // equip
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetCost(s.spcost)
    effect::set_cost(d, e5, Some(cost_target(spcost)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.regfilter(c,ec)`
fn regfilter(d: &mut Duel, c: CardId, ec: Option<CardId>) -> bool {
    // return c==ec and c:GetBattleTarget():GetPreviousLevelOnField()>=5
    if Some(c) != ec {
        return false;
    }
    let bt = card::get_battle_target(d, c).nil();
    card::get_previous_level_on_field(d, bt) >= 5
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.regfilter,1,nil,e:GetHandler():GetEquipTarget()) then
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    let ec = card::get_equip_target(d, h);
    if group::is_exists(d, &eg, &|d, c| regfilter(d, c, ec), 1, NO_EX) {
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // if chk==0 then return ec:IsReleasable() end
    if a.chk == Some(0) {
        return card::is_releasable(d, ec, None, None);
    }
    // e:SetLabel(ec:GetCode())
    let codes: Vec<i64> = card::get_code(d, ec).into_iter().map(|x| x as i64).collect();
    effect::set_label(d, a.e, &codes);
    // Duel.Release(ec,REASON_COST)
    duel::release(d, ec, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: i64) -> bool {
    // return ((code==81383947 and c:IsCode(75917088)) or (code==46128076 and c:IsCode(2316186)))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    ((code == 81383947 && card::is_code(d, c, &[75917088])) || (code == 46128076 && card::is_code(d, c, &[2316186])))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,e:GetHandler():GetEquipTarget():GetCode()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let ec = card::get_equip_target(d, h).nil();
        let code = card::get_code(d, ec).first().copied().unwrap_or(0) as i64;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e);
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.SetTargetParam(code)
    duel::set_target_param(d, code as u32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,code)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g.clone(), 0, tp, tp, true, false, POS_FACEUP as u8, None);
        // g:GetFirst():CompleteProcedure()
        let first = group::get_first(d, &g).nil();
        card::complete_procedure(d, first);
    }
}
