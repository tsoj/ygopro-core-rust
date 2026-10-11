//! Blink Out
use super::prelude::*;

pub const ID: u32 = 28279365;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 Link monster to the Extra Deck, then you can Special Summon 1 material from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkMonster() and c:IsAbleToExtra()
    card::is_link_monster(d, c) && card::is_able_to_extra(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(g), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
// function s.mgfilter(c,e,tp,link)
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, link: CardId) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    //     and (c:GetReason()&(REASON_MATERIAL|REASON_LINK))==REASON_MATERIAL|REASON_LINK and c:GetReasonCard()==link
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & (REASON_MATERIAL | REASON_LINK)) == REASON_MATERIAL | REASON_LINK
        && card::get_reason_card(d, c) == Some(link)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local mg=tc:GetMaterial()
    let mg = card::get_material(d, tc);
    // local ct=#mg
    let ct = mg.len();
    // local sumtype=tc:GetSummonType()
    let sumtype = card::get_summon_type(d, tc);
    let nf = aux::necro_valley_filter(move |d: &mut Duel, c: CardId| mgfilter(d, c, e, tp, tc));
    // if Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)>0 and (sumtype&SUMMON_TYPE_LINK)==SUMMON_TYPE_LINK
    //     and ct>0 and tc:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and mg:IsExists(aux.NecroValleyFilter(s.mgfilter),1,nil,e,tp,tc)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
        && (sumtype & SUMMON_TYPE_LINK) == SUMMON_TYPE_LINK
        && ct > 0
        && card::is_location(d, tc, LOCATION_EXTRA)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && group::is_exists(d, &mg, &nf, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=mg:FilterSelect(tp,aux.NecroValleyFilter(s.mgfilter),1,1,nil,e,tp,tc)
        let sg = group::filter_select(d, &mg, tp, &nf, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
