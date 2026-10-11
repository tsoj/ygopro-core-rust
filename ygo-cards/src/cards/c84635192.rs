//! Tsuru-Puru-Purun
use super::prelude::*;

pub const ID: u32 = 84635192;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsFaceup()
    //     and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_TOFIELD,1<<c:GetSequence())>0
    card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_faceup(d, c)
        && {
            let zone = 1u32 << card::get_sequence(d, c);
            duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
        }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MMZONE) and chkc:IsControler(tp) and s.spfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MMZONE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_MMZONE,0,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.tp)), a.tp, LOCATION_MMZONE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_MMZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.tp)), a.tp, LOCATION_MMZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsControler(tp)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, a.tp)) {
        return;
    }
    // local seq=tc:GetSequence()
    let seq = card::get_sequence(d, tc);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.Destroy(tc,REASON_EFFECT)>0 and c:IsRelateToEffect(e)
    //     and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,1<<seq)>0 then
    if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(1u32 << seq)) > 0
    {
        // local g=c:GetColumnGroup():Filter(aux.FaceupFilter(Card.IsControler,1-tp),nil)
        let cg = card::get_column_group(d, c, 0, 0);
        let g = group::filter(d, &cg, &aux::faceup_filter(|d, x| card::is_controler(d, x, 1 - a.tp)), NO_EX);
        // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
        if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
