//! Borrelend Dragon
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 98630720;

// s.listed_series={SET_ROKKET}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROKKET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 3+ Effect Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 3, None, None, None, None);
    // Cannot be destroyed by battle or card effects, also neither player can target this card with monster effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1a, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetValue(1)
    effect::set_value(d, e1a, 1);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1b, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    let e1c = effect::clone(d, e1a);
    // e1c:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1c, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1c:SetValue(function(e,re,rp) return re:IsMonsterEffect() end)
    effect::set_value_fn(d, e1c, lua_fn(|d, a: Args| effect::is_monster_effect(d, a.e(1))));
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // Can attack all monsters your opponent controls, once each
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e2, EFFECT_ATTACK_ALL);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // (Quick Effect): You can target 1 Effect Monster on the field and 1 "Rokket" monster in your GY; negate the effects of that monster on the field, and if you do, Special Summon that other monster from your GY. Your opponent cannot activate cards or effects in response to this effect's activation. You can only use this effect of "Borrelend Dragon" once per turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DISABLE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DISABLE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ROKKET) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ROKKET as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(aux.AND(Card.IsEffectMonster,Card.IsNegatableMonster),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_effect_monster(d, c) && card::is_negatable_monster(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g1=Duel.SelectTarget(tp,aux.AND(Card.IsEffectMonster,Card.IsNegatableMonster),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g1 = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_effect_monster(d, c) && card::is_negatable_monster(d, c)),
        tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g2 = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetChainLimit(function(e,ep,tp) return tp==ep end)
    duel::set_chain_limit(d, lua_fn(|_d, x: Args| x.p(2) == x.p(1)));
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g1,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g1), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g2,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g2), 1, tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local field_tc=tg:Filter(Card.IsLocation,nil,LOCATION_MZONE):GetFirst()
    let fg = group::filter(d, &tg, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    let field_tc = group::get_first(d, &fg);
    // if field_tc and field_tc:IsMonster() and field_tc:IsFaceup() and field_tc:IsCanBeDisabledByEffect(e) then
    if let Some(field_tc) = field_tc {
        if card::is_monster(d, field_tc) && card::is_faceup(d, field_tc) && card::is_can_be_disabled_by_effect(d, field_tc, a.e, true) {
            // Negate the effects of that monster on the field
            // field_tc:NegateEffects(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            card::negate_effects(d, field_tc, h, None, false, None);
            // Duel.AdjustInstantly(field_tc)
            duel::adjust_instantly(d, Some(field_tc));
            // local gy_tc=tg-field_tc
            let mut gy_tc = tg.clone();
            group::remove_card(&mut gy_tc, &CardSet::from_card(field_tc));
            // if field_tc:IsDisabled() and #gy_tc>0 then
            if card::is_disabled(d, field_tc) && !gy_tc.is_empty() {
                // Duel.SpecialSummon(gy_tc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, gy_tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
