//! Super Duper Polymerization
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 44886582;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON|CATEGORY_FUSION_SUMMON|CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_FUSION_SUMMON | CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:HasLevel()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EFFECT)
        && card::has_level(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,tc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    // return c:IsRace(tc:GetRace()) and c:IsAttribute(tc:GetAttribute())
    //     and c:HasLevel() and not c:IsLevel(tc:GetLevel())
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    let race = card::get_race(d, tc);
    let attribute = card::get_attribute(d, tc);
    let level = card::get_level(d, tc);
    card::is_race(d, c, race)
        && card::is_attribute(d, c, attribute)
        && card::has_level(d, c)
        && !card::is_level(d, c, &[level])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp)
    let _ = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(aux.FALSE)
        duel::set_chain_limit(d, lua_fn(|_d, _a| false));
    }
    true
}
/// `s.fextra(tc)`: `function(e,tp,mg) return Group.FromCards(tc) end`
fn fextra(tc: CardId) -> fusion::ExtraFil {
    Arc::new(move |_d, _e, _tp, _mg| (Some(CardSet::from_card(tc)), None))
}
/// `gc = Group.FromCards(tc,sc)`
fn gc_of(tc: CardId, sc: CardId) -> fusion::Gc {
    let mut g = CardSet::from_card(tc);
    group::add_card(&mut g, &CardSet::from_card(sc));
    Arc::new(move |_d, _a| Value::Group(g.clone()))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = group::get_first(d, &sg) {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            let c = effect::get_handler(d, e).nil();
            // sc:NegateEffects(e:GetHandler())
            card::negate_effects(d, sc, c, None, false, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local fusion_params={matfilter=Fusion.OnFieldMat,extrafil=s.fextra(tc),exactcount=2,gc=Group.FromCards(tc,sc)}
            let params = fusion::SummonEff {
                matfilter: Some(fusion::on_field_mat()),
                extrafil: Some(fextra(tc)),
                exactcount: Some(2),
                gc: Some(gc_of(tc, sc)),
                ..Default::default()
            };
            // local b1=Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
            let b1 = fusion::summon_eff_tg_call(d, &params, &EffArgs { chk: Some(0), ..a.clone() });
            // local b2=sc:IsAbleToGrave()
            let b2 = card::is_able_to_grave(d, sc, None);
            // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
            let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
            if op == 1 {
                // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
                fusion::summon_eff_op_call(d, &params, a);
            } else if op == 2 {
                // Duel.SendtoGrave(sc,REASON_EFFECT)
                duel::send_to_grave(d, sc, REASON_EFFECT, None, None);
            }
        }
    }
}
