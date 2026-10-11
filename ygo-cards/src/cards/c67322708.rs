//! D/D Lance Soldier
use super::prelude::*;

pub const ID: u32 = 67322708;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD, SET_DARK_CONTRACT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase the Level of 1 "D/D" monster you control by up to the number of "Dark Contract" cards in your field and GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 "Dark Contract" card you control, and if you do, Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DD) and c:HasLevel() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_DD as u16]) && card::has_level(d, c) && card::is_faceup(d, c)
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_DARK_CONTRACT)`
fn dc_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.lvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && lvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DARK_CONTRACT),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&lvfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&dc_faceup), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&lvfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) or not tc:HasLevel() then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) || !card::has_level(d, tc) {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_DARK_CONTRACT),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&dc_faceup), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local lv=Duel.AnnounceLevel(tp,1,ct)
    let lv = duel::announce_number_range(d, tp, Some(1), Some(ct as u32), &[]).nil().0 as i64;
    // if lv then
    // tc:UpdateLevel(lv,nil,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    card::update_level(d, tc, lv, None, Some(h));
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DARK_CONTRACT) and c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0
    card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])
        && card::is_faceup(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsOnField() and s.desfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_on_field(d, chkc) && desfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and c:IsRelateToEffect(e)
    //     and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // Banish it when it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
