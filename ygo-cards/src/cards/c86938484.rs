//! Shaddoll Construct
use super::prelude::*;

pub const ID: u32 = 86938484;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_FLIP),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_FLIP, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Fusion summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // local params = {aux.FilterBoolFunction(Card.IsSetCard,SET_SHADDOLL)}
    // e1:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    // e1:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let p = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_SHADDOLL as u16]))),
        ..Default::default()
    };
    let tg = fusion::summon_eff_tg(d, p.clone());
    effect::set_target(d, e1, Some(tg));
    let op = fusion::summon_eff_op(d, p);
    effect::set_operation(d, e1, Some(op));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsSetCard(SET_SHADDOLL) and c:IsAbleToGraveAsCost()
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c)) && card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.mzfilter(c)`
fn mzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5
    card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local sg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,nil)
    let sg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and #sg>=1 and (ft>0 or sg:IsExists(s.mzfilter,1,nil)) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
            && !sg.is_empty()
            && (ft > 0 || group::is_exists(d, &sg, &mzfilter, 1, NO_EX));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local sg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,nil)
    let sg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, NO_EX);
    // local g=nil
    let g = if ft <= 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // g=sg:FilterSelect(tp,s.mzfilter,1,1,nil)
        group::filter_select(d, &sg, tp, &mzfilter, 1, 1, false, NO_EX).nil()
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // g=sg:Select(tp,1,1,nil)
        group::select(d, &sg, tp, 1, 1, false, NO_EX).nil()
    };
    // if Duel.SendtoGrave(g,REASON_EFFECT)~=0 and g:GetFirst():IsLocation(LOCATION_GRAVE) then
    if duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) != 0 {
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
