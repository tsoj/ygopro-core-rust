//! Amazoness Pet Liger King
use super::prelude::*;

pub const ID: u32 = 59353647;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.mfilter,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_AMAZONESS))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| mfilter(d, c, Some(fc), st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_set_card_ctx(d, c, &[SET_AMAZONESS as u16], Some(fc), st, Some(tp))
            })),
        ],
    );
    // Cannot attack monsters, except this one
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetValue(function(e,c) return c~=e:GetHandler() end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| c != effect::get_handler(d, e).nil()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 "Amazoness" card and Special Summon from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mfilter(c,sc,st,tp)`
fn mfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_AMAZONESS,sc,st,tp) and c:IsLevelAbove(5)
    card::is_set_card_ctx(d, c, &[SET_AMAZONESS as u16], sc, st, Some(tp)) && card::is_level_above(d, c, 5)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, c) == 0;
    }
    // Cannot attack this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    true
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AMAZONESS) and Duel.GetMZoneCount(tp,c)>0
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_AMAZONESS as u16])
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsSetCard(SET_AMAZONESS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WARRIOR as u64)
        && card::is_set_card(d, c, &[SET_AMAZONESS as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil,tp)
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g2 = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g1.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g2, 1, 0, 0);
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1.nil());
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local tc1=tg:GetFirst()
    // local tc2=tg:GetNext()
    let order = group::iter(d, &tg);
    let mut tc1 = order.first().copied();
    let mut tc2 = order.get(1).copied();
    // if tc1~=e:GetLabelObject() then tc1,tc2=tc2,tc1 end
    if tc1 != effect::get_label_object(d, a.e).as_card() {
        std::mem::swap(&mut tc1, &mut tc2);
    }
    // if tc1 and Duel.Destroy(tc1,REASON_EFFECT)>0 and tc2 then
    if let Some(tc1) = tc1 {
        if duel::destroy(d, CardSet::from_card(tc1), REASON_EFFECT, None, None) > 0 {
            if let Some(tc2) = tc2 {
                // Duel.SpecialSummon(tc2,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, CardSet::from_card(tc2), 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
