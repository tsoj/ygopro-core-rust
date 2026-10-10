//! Brigrand the Glory Dragon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 34848821;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TRI_BRIGADE],
    listed_names: &[CARD_ALBAZ],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,aux.FilterBoolFunctionEx(Card.IsLevelAbove,8))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_ALBAZ),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_level_above(d, c, 8))),
        ],
    );
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Limit target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.tgcond)
    effect::set_condition(d, e2, Some(condition(tgcond)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(effect_card_fn(tgtg)));
    // e2:SetValue(s.tgval)
    effect::set_value_fn(d, e2, lua_fn(tgval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register when sent to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Search or Special Summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.condition)
    effect::set_condition(d, e4, Some(condition(cond)));
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tgcond(e,tp,eg,ep,ev,re,r,rp)`
fn tgcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.tgtg(e,c)`
fn tgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    c != effect::get_handler(d, e).nil()
}
/// `s.tgval(e,re,rp)`
fn tgval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    let rp = a.p(2);
    // return re:IsMonsterEffect() and aux.tgoval(e,re,rp)
    effect::is_monster_effect(d, re) && aux::tgoval().call(d, &[Value::Effect(e), Value::Effect(re), Value::Int(rp as i64)]).truthy()
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsCode(CARD_ALBAZ) or (c:IsSetCard(SET_TRI_BRIGADE) and c:IsMonster()))
    //     and (c:IsAbleToHand() or (c:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0))
    (card::is_code(d, c, &[CARD_ALBAZ]) || (card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16]) && card::is_monster(d, c)))
        && (card::is_able_to_hand(d, c, None)
            || (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,0,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 0, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // tc=g:GetFirst()  (a Lua global)
        let tc = group::get_first(d, &g).nil();
        set_lua_global(d, "tc", Value::Card(tc));
        let (e, tp) = (a.e, a.tp);
        // aux.ToHandOrElse(tc,tp,
        //     function(c) return tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end,
        //     function(c) Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,2))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, _c| card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0),
            Some(&|d, _g| duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
            Some(aux::stringid(ID, 2)),
        );
    }
}
