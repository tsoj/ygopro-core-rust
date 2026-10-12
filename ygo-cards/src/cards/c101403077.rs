//! Splendor of the White Crow Witch
//! Scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 101403077;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WHITE_CROW, SET_DIABELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can activate 1 of these effects;
    // ● Special Summon 1 "White Crow" monster from your hand or GY
    // ● Fusion Summon 1 "Diabell" Fusion Monster from your Extra Deck, using up to 1 "White Crow" monster each from your hand, Extra Deck, and/or field
    // ● If you control a "Diabell" monster: Target 1 face-up card on the field; negate its effects until the end of this turn
    // You can only use this effect of "Splendor of the White Crow Witch" once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_WHITE_CROW) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_WHITE_CROW as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.locationcheck(tp,sg,fc)`
fn locationcheck(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    // return #sg<=3 and sg:GetClassCount(Card.GetLocation)==#sg
    sg.len() <= 3 && group::get_class_count(d, &sg, &|d, c| card::get_location(d, c) as i64) == sg.len()
}
/// `s.extramaterialgroup(e,tp,mg)`
fn extramaterialgroup(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_EXTRA,0,nil),s.locationcheck
    let f = fusion::is_monster_filter(fusion::fus_filter(|d, c, _tp| card::is_able_to_grave(d, c, None)));
    let g = duel::get_matching_group(d, Some(&|d, c| f(d, c, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    (Some(g), Some(lua_fn(locationcheck)))
}
/// The `fusion_params` table of the two Fusion Summon options.
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=function(c) return c:IsSetCard(SET_DIABELL) end
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DIABELL as u16]))),
        // matfilter=function(c) return c:IsSetCard(SET_WHITE_CROW) end
        matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_set_card(d, c, &[SET_WHITE_CROW as u16]))),
        // extrafil=s.extramaterialgroup
        extrafil: Some(Arc::new(extramaterialgroup)),
        ..Default::default()
    }
}
/// The arguments of a Lua call of an effect function (`chk` = the chk value).
fn args(a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    v
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return e:GetChainData().choice==3 and chkc:IsOnField() and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        let choice = chain::effect_data(d, a.e, None, chain::field::CARD);
        return matches!(choice, Value::Int(3)) && card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    // ● Special Summon 1 "White Crow" monster from your hand or GY
    // local option_1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
    let option_1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    // local fusion_params={...}
    let fusion_tg = fusion::summon_eff_tg(d, fusion_params());
    // local option_2=Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let option_2 = fusion_tg.call(d, &args(a, Some(0))).truthy();
    // local option_3=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DIABELL),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let option_3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DIABELL as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // if chk==0 then return option_1 or option_2 or option_3 end
    if a.chk == Some(0) {
        return option_1 || option_2 || option_3;
    }
    // local choice=Duel.SelectEffect(tp,
    //     {option_1,aux.Stringid(id,1)},
    //     {option_2,aux.Stringid(id,2)},
    //     {option_3,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, a.tp, &[(option_1, aux::stringid(ID, 1)), (option_2, aux::stringid(ID, 2)), (option_3, aux::stringid(ID, 3))]);
    // e:GetChainData().choice=choice
    let choice_v = match choice {
        Some(x) => Value::Int(x as i64),
        None => Value::Nil,
    };
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_v);
    // if choice==1 then
    if choice == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // elseif choice==2 then
    } else if choice == Some(2) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,1)
        fusion_tg.call(d, &args(a, Some(1)));
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    // elseif choice==3 then
    } else if choice == Some(3) {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
        // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local choice=e:GetChainData().choice
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD);
    // if choice==1 then
    if matches!(choice, Value::Int(1)) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif choice==2 then
    } else if matches!(choice, Value::Int(2)) {
        // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
        let op = fusion::summon_eff_op(d, fusion_params());
        op.call(d, &args(a, None));
    // elseif choice==3 then
    } else if matches!(choice, Value::Int(3)) {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END,true)
            let h = effect::get_handler(d, a.e).nil();
            card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), true, None);
        }
    }
}
