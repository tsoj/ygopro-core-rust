//! Starliege Lord Galaxion
use super::prelude::*;

pub const ID: u32 = 40390147;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_GALAXYEYES_P_DRAGON],
    listed_series: &[SET_PHOTON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 "Photon" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PHOTON),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_PHOTON as u16], sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Special Summon 1 "Galaxy-Eyes Photon Dragon" from your hand or Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.Choice(
    //     {Cost.DetachFromSelf(1),aux.Stringid(id,1),s.spcheck(LOCATION_HAND)},
    //     {Cost.DetachFromSelf(2),aux.Stringid(id,2),s.spcheck(LOCATION_DECK)}
    // ))
    effect::set_cost(
        d,
        e1,
        Some(cost::choice(vec![
            cost::CostChoice {
                cost: cost::detach_from_self(cost::Count::N(1), None, None),
                desc: aux::stringid(ID, 1),
                check: Some(spcheck(LOCATION_HAND)),
            },
            cost::CostChoice {
                cost: cost::detach_from_self(cost::Count::N(2), None, None),
                desc: aux::stringid(ID, 2),
                check: Some(spcheck(LOCATION_DECK)),
            },
        ])),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_GALAXYEYES_P_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcheck(loc)`: returns `function(e,tp) return Duel.IsExistingMatchingCard(...) end`
fn spcheck(loc: u32) -> LuaFn {
    lua_fn(move |d, a: Args| {
        let (e, tp) = (a.e(0), a.p(1));
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, loc, 0, 1, NO_EX)
    })
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // local loc=e:GetChainData().cost_choice==1 and LOCATION_HAND or LOCATION_DECK
    let loc = if chain::effect_data(d, a.e, None, chain::field::COST_CHOICE).int() == 1 { LOCATION_HAND } else { LOCATION_DECK };
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, loc as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local loc=e:GetChainData().cost_choice==1 and LOCATION_HAND or LOCATION_DECK
    let loc = if chain::effect_data(d, e, None, chain::field::COST_CHOICE).int() == 1 { LOCATION_HAND } else { LOCATION_DECK };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,loc,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, loc, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
