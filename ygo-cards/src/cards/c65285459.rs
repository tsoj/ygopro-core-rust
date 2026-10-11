//! Sunavalon Dryanome
use super::prelude::*;

pub const ID: u32 = 65285459;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUNVINE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PLANT),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PLANT, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // attack target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IGNORE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_IGNORE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(3)
    effect::set_count_limit(d, e2, 3, 0, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate an attack and move that monster you control to another of your Main Monster Zones
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.mvcon)
    effect::set_condition(d, e3, Some(condition(mvcon)));
    // e3:SetOperation(s.mvop)
    effect::set_operation(d, e3, Some(operation(mvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and (r&REASON_BATTLE+REASON_EFFECT)~=0
    a.ep == a.tp && (a.r & (REASON_BATTLE + REASON_EFFECT)) != 0
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SUNVINE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SUNVINE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCountFromEx(tp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,1,tp,ev)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 1, tp, a.ev as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let ev = a.ev as i64;
    // if Duel.Recover(tp,ev,REASON_EFFECT)==ev and Duel.GetLocationCountFromEx(tp)>0 then
    if duel::recover(d, tp, ev, REASON_EFFECT, false, None) as i64 == ev
        && duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// function s.mvcon(e)
fn mvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandler():GetControler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = card::get_controler(d, c);
    // local lg=e:GetHandler():GetLinkedGroup()
    let lg = card::get_linked_group(d, c);
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return d and lg:IsContains(d) and d:IsControler(tp) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    match at {
        Some(x) => {
            lg.contains(x)
                && card::is_controler(d, x, tp)
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        }
        None => false,
    }
}
// function s.mvop(e,tp,eg,ep,ev,re,r,rp)
fn mvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // if tc and tc:IsRelateToBattle() and Duel.NegateAttack() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc)
            && duel::negate_attack(d)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // local s=Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,nil)
            let s = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, None, false);
            // local nseq=math.log(s,2)
            if s == 0 {
                // math.log(nil,2): no zone to choose
                crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
            }
            let nseq = s.trailing_zeros() as u8;
            // Duel.MoveSequence(tc,nseq)
            duel::move_sequence(d, tc, nseq, None);
        }
    }
}
