//! Multiple Piece Golem
//! マルチ・ピース・ゴーレム
use super::prelude::*;

pub const ID: u32 = 71628381;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,25247218,58843503)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(25247218), fusion::Mat::Code(58843503)]);
    // Return itself to the Extra Deck and special summon materials
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let c = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, c) > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToDeck() end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.mgfilter(c,e,tp,fusc)`
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fusc: CardId) -> bool {
    // return not c:IsControler(tp) or not c:IsLocation(LOCATION_GRAVE)
    //     or (c:GetReason()&(REASON_FUSION|REASON_MATERIAL))~=(REASON_FUSION|REASON_MATERIAL) or c:GetReasonCard()~=fusc
    //     or not c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    !card::is_controler(d, c, tp)
        || !card::is_location(d, c, LOCATION_GRAVE)
        || (card::get_reason(d, c) & (REASON_FUSION | REASON_MATERIAL)) != (REASON_FUSION | REASON_MATERIAL)
        || card::get_reason_card(d, c) != Some(fusc)
        || !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
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
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // local sumable=true
    let mut sumable = true;
    // local sumtype=c:GetSummonType()
    let sumtype = card::get_summon_type(d, c);
    // if Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_EFFECT)==0 or (sumtype&SUMMON_TYPE_FUSION)~=SUMMON_TYPE_FUSION or #mg==0
    //     or #mg>Duel.GetLocationCount(tp,LOCATION_MZONE)
    //     or mg:IsExists(s.mgfilter,1,nil,e,tp,c) then
    if duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) == 0
        || (sumtype & SUMMON_TYPE_FUSION) != SUMMON_TYPE_FUSION
        || mg.is_empty()
        || mg.len() as i32 > duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
        || group::is_exists(d, &mg, &|d, x| mgfilter(d, x, a.e, tp, c), 1, NO_EX)
    {
        // sumable=false
        sumable = false;
    }
    // if sumable and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if sumable && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(mg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, mg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
