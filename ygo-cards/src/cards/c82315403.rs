//! Cyber Eternity Dragon
use super::prelude::*;

pub const ID: u32 = 82315403;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CYBER_DRAGON],
    listed_names: &[CARD_CYBER_DRAGON],
    material_setcode: &[SET_CYBER, SET_CYBER_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_CYBER_DRAGON),1,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_CYBER_DRAGON as u16], Some(fc), st, Some(tp)))), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_MACHINE, Some(fc), st, Some(tp)))), 2),
        ],
    );
    // Cannot be target
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e0:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e0, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetCondition(s.imunecond)
    effect::set_condition(d, e0, Some(condition(imunecond)));
    // e0:SetValue(aux.tgoval)
    effect::set_value_fn(d, e0, aux::tgoval());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot be destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.imunecond)
    effect::set_condition(d, e1, Some(condition(imunecond)));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // protection
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.imunecond(e)
fn imunecond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&filter), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsRace(RACE_MACHINE)
    card::is_type(d, c, TYPE_FUSION) && card::is_race(d, c, RACE_MACHINE)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFusionSummoned() and c:IsPreviousControler(tp) and rp==1-tp
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_fusion_summoned(d, c) && card::is_previous_controler(d, c, a.tp) && a.rp == 1 - a.tp
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_CYBER_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_CYBER_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let e = a.e;
    let tp = a.tp;
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,true,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None);
    }
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(effect_card_fn(tg)));
    // e1:SetValue(s.tgvalue)
    effect::set_value_fn(d, e1, lua_fn(tgvalue));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
// function s.tgvalue(e,re,rp)
fn tgvalue(d: &mut Duel, a: Args) -> bool {
    // return rp==1-e:GetHandlerPlayer()
    let e = a.e(0);
    let hp = effect::get_handler_player(d, e);
    a.p(2) == 1 - hp
}
// function s.tg(e,c)
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION)
}
