//! Abyss Actor - Hyper Director
//! --魔界劇団-ハイパー・ディレクター
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 2368215;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ABYSS_ACTOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_ABYSS_ACTOR,lc,sumtype,tp) and c:IsType(TYPE_PENDULUM,lc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_ABYSS_ACTOR as u16], lc, sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_PENDULUM, lc, sumtype, Some(tp))
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.pfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,c:GetCode())
    // The Lua passes 1 as the exception argument (a number, so no exception) and the code after
    // the nil: pfilter receives nil as its cd argument.
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| pfilter(d, x, None)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
fn pfilter(d: &mut Duel, c: CardId, cd: Option<u32>) -> bool {
    // return c:IsSetCard(SET_ABYSS_ACTOR) and c:IsType(TYPE_PENDULUM)
    //     and (c:IsFaceup() or not c:IsLocation(LOCATION_EXTRA))
    //     and not c:IsForbidden() and not c:IsCode(cd)
    card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_EXTRA))
        && !card::is_forbidden(d, c)
        && !cd.is_some_and(|x| card::is_code(d, c, &[x]))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_PZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_PZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, tc, e)
        && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local g=Duel.SelectMatchingCard(tp,s.pfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,tc:GetCode())
        let code = card::get_code(d, tc)[0];
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| pfilter(d, x, Some(code))),
            tp,
            LOCATION_DECK | LOCATION_EXTRA,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.MoveToField(g:GetFirst(),tp,tp,LOCATION_PZONE,POS_FACEUP,true)
            let first = group::get_first(d, &g).nil();
            duel::move_to_field(d, first, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let owner = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, owner);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, owner);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_ABYSS_ACTOR)
    !card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
}
