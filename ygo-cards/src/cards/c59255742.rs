//! Goyo Emperor
use super::prelude::*;

pub const ID: u32 = 59255742;

pub const INFO: ScriptInfo = ScriptInfo { miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))), 2)]);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCondition(s.spcon2)
    effect::set_condition(d, e2, Some(condition(spcon2)));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // take control
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e3, CATEGORY_CONTROL);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost_fn)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // return control
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetCondition(s.ctcon)
    effect::set_condition(d, e4, Some(condition(ctcon)));
    // e4:SetTarget(s.cttg)
    effect::set_target(d, e4, Some(cost_target(cttg)));
    // e4:SetOperation(s.ctop)
    effect::set_operation(d, e4, Some(operation(ctop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.ffilter(c,fc,sumtype,tp)
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH,fc,sumtype,tp) and c:IsRace(RACE_WARRIOR,fc,sumtype,tp) and c:IsType(TYPE_SYNCHRO,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_EARTH, Some(fc), sumtype, Some(tp))
        && card::is_race_ctx(d, c, RACE_WARRIOR, Some(fc), sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(fc), sumtype, Some(tp))
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if not c:IsRelateToBattle() or c:IsFacedown() then return false end
    if !card::is_relate_to_battle(d, c) || card::is_facedown(d, c) {
        return false;
    }
    let bc = bc.nil();
    // return bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and bc:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, bc.nil(), a.e, 0, tp, false, false, None, None, None);
    }
    let bc = bc.nil();
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
// function s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // local rc=tc:GetReasonCard()
    let rc = card::get_reason_card(d, tc).nil();
    // local bc=rc:GetBattleTarget()
    let bc = card::get_battle_target(d, rc);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and bc:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, bc.nil(), a.e, 0, tp, false, false, None, None, None);
    }
    let bc = bc.nil();
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.spcon2(e,tp,eg,ep,ev,re,r,rp)
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &eg).nil();
    // local rc=tc:GetReasonCard()
    let rc = card::get_reason_card(d, tc).nil();
    // return #eg==1 and rc:IsControler(tp) and rc:GetOwner()~=tp
    //     and tc:IsMonster() and tc:IsReason(REASON_BATTLE) and tc:IsLocation(LOCATION_GRAVE)
    eg.len() == 1
        && card::is_controler(d, rc, tp)
        && card::get_owner(d, rc) != tp
        && card::is_monster(d, tc)
        && card::is_reason(d, tc, REASON_BATTLE)
        && card::is_location(d, tc, LOCATION_GRAVE)
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and c:IsType(TYPE_SYNCHRO)
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_WARRIOR as u64) && card::is_type(d, c, TYPE_SYNCHRO)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsControlerCanBeChanged()
    card::is_summon_player(d, c, 1 - tp) && card::is_controler_can_be_changed(d, c, false, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=eg:Filter(s.filter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, tp), NO_EX);
    // if chk==0 then return #g>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>=#g-1 end
    if a.chk == Some(0) {
        return !g.is_empty() && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= g.len() as i32 - 1;
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), n, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<#g then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < g.len() as i32 {
        return;
    }
    // Duel.GetControl(g,tp)
    duel::get_control(d, g, tp, 0, 0, None, None);
}
// function s.ctcon(e,tp,eg,ep,ev,re,r,rp)
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEUP) and c:GetLocation()~=LOCATION_DECK
    card::is_previous_position(d, c, POS_FACEUP) && card::get_location(d, c) != LOCATION_DECK
}
// function s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetControler()~=c:GetOwner()
    card::get_controler(d, c) != card::get_owner(d, c)
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ctfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ctfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if not tc:IsImmuneToEffect(e) then
        if !card::is_immune_to_effect(d, tc, a.e) {
            // tc:ResetEffect(EFFECT_SET_CONTROL,RESET_CODE)
            card::reset_effect(d, tc, EFFECT_SET_CONTROL, RESET_CODE);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_CONTROL)
            effect::set_code(d, e1, EFFECT_SET_CONTROL);
            // e1:SetValue(tc:GetOwner())
            let owner = card::get_owner(d, tc);
            effect::set_value(d, e1, owner as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_TEMP_REMOVE|RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_TEMP_REMOVE | RESET_TURN_SET)), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
