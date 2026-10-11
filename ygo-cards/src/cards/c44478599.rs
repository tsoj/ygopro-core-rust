//! Sunavalon Melias
use super::prelude::*;

pub const ID: u32 = 44478599;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SUNVINE, SET_SUNAVALON],
    listed_names: &[27520594],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PLANT),2,nil,s.matcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PLANT, sc, st, Some(tp)))),
        2,
        None,
        Some(group_check(matcheck)),
        None,
        None,
    );
    // Special Summon 1 "Sunseed Genius Loci" from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            card::is_link_summoned(d, c)
        })),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be attack target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IGNORE_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_IGNORE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Multi Attacks
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function() return Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e3, Some(condition(|d, _a| duel::is_able_to_enter_bp(d))));
    // e3:SetTarget(s.attg)
    effect::set_target(d, e3, Some(cost_target(attg)));
    // e3:SetOperation(s.atop)
    effect::set_operation(d, e3, Some(operation(atop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.matcheck(g,lc,sumtype,tp)
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_LINK,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_LINK, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(27520594) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[27520594]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.atfilter(c,lg)
fn atfilter(d: &mut Duel, c: CardId, lg: &CardSet) -> bool {
    // return c:IsSetCard(SET_SUNVINE) and c:IsLinkMonster() and lg and lg:IsContains(c)
    card::is_set_card(d, c, &[SET_SUNVINE as u16]) && card::is_link_monster(d, c) && lg.contains(c)
}
// function s.valfilter(c)
fn valfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SUNAVALON) and c:IsLinkMonster()
    card::is_set_card(d, c, &[SET_SUNAVALON as u16]) && card::is_link_monster(d, c)
}
// function s.attg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local lg=e:GetHandler():GetLinkedGroup()
    let c = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, c);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.atfilter(chkc,lg) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && atfilter(d, chkc, &lg);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atfilter,tp,LOCATION_MZONE,0,1,nil,lg)
    //     and Duel.GetMatchingGroupCount(s.valfilter,tp,LOCATION_MZONE,0,nil)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| atfilter(d, x, &lg)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::get_matching_group_count(d, Some(&valfilter), tp, LOCATION_MZONE, 0, NO_EX) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.atfilter,tp,LOCATION_MZONE,0,1,1,nil,lg)
    duel::select_target(d, tp, Some(&|d, x| atfilter(d, x, &lg)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.atop(e,tp,eg,ep,ev,re,r,rp)
fn atop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(s.valfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&valfilter), a.tp, LOCATION_MZONE, 0, NO_EX) as i64;
    if ct > 0 {
        // Can make additional attacks this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(ct-1)
        effect::set_value(d, e1, ct - 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    } else if ct == 0 {
        // Cannot attack this turn if there are no "Sunavalon" Link Monsters when this effect resolves
        // local e2=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
    // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
    card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
}
