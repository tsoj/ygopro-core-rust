//! Salamangreat Jack Jaguar
//! scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 56003780;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Piercing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle target to Deck and Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,e,tp,sc)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SALAMANGREAT) and c:IsLinkMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_monster(d, c) && card::is_faceup(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SALAMANGREAT) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_deck(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && tdfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=aux.GetMMZonesPointedTo(tp,Card.IsSetCard,LOCATION_MZONE,0,nil,SET_SALAMANGREAT)
    let zone = aux::get_mm_zones_pointed_to(d, tp, Some(&|d, x| card::is_set_card(d, x, &[SET_SALAMANGREAT as u16])), Some(LOCATION_MZONE), Some(0), None);
    // if chk==0 then return zone>0 and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,c)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone) end
    if a.chk == Some(0) {
        return zone > 0
            && duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, c.into())
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local zone=aux.GetMMZonesPointedTo(tp,Card.IsSetCard,LOCATION_MZONE,0,nil,SET_SALAMANGREAT)
    let zone = aux::get_mm_zones_pointed_to(d, tp, Some(&|d, x| card::is_set_card(d, x, &[SET_SALAMANGREAT as u16])), Some(LOCATION_MZONE), Some(0), None);
    // if tc:IsRelateToEffect(e) and s.tdfilter(tc) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    //     and c:IsRelateToEffect(e) and zone>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && tdfilter(d, tc)
        && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
        && card::is_relate_to_effect(d, c, a.e)
        && zone > 0
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
