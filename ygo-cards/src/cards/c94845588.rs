//! The Hallowed Azamina
//! 聖なる薊花
use super::prelude::*;

pub const ID: u32 = 94845588;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_AZAMINA, SET_SINFUL_SPOILS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send "Sinful Spoils" cards to the GY and Special Summon 1 "Azamina" Fusion Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 "Azamina" monster into the Deck and add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sinfilter(c)`
fn sinfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SINFUL_SPOILS) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_SINFUL_SPOILS as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.spfilter(c,e,tp,lv,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32, g: &CardSet) -> bool {
    // return c:IsSetCard(SET_AZAMINA) and c:IsType(TYPE_FUSION) and c:IsLevelAbove(4) and c:IsLevelBelow(lv) and not c:IsPublic()
    //     and Duel.GetLocationCountFromEx(tp,tp,g,c)>0 and c:CheckFusionMaterial()
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false)
    card::is_set_card(d, c, &[SET_AZAMINA as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_level_above(d, c, 4)
        && card::is_level_below(d, c, lv)
        && !card::is_public(d, c)
        && duel::get_location_count_from_ex(d, tp, Some(tp), g.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::check_fusion_material(d, c, None, None, None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.sinfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&sinfilter), tp, LOCATION_ONFIELD + LOCATION_HAND, 0, NO_EX);
        // local ct=#g
        let ct = g.len() as u32;
        // return ct>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,ct*4+3,g)
        return ct > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, ct * 4 + 3, &g)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_ONFIELD|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_ONFIELD + LOCATION_HAND) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.rescon(fusc)`: the check of the material group of the Fusion Monster `fusc`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, fusc: CardId) -> (bool, bool) {
    // return function(sg,e,tp,mg)
    //     return Duel.GetLocationCountFromEx(tp,tp,sg,fusc)>0
    let ok = duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(fusc), None).0 > 0;
    (ok, false)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local sg=Duel.GetMatchingGroup(s.sinfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil)
    let sg = duel::get_matching_group(d, Some(&sinfilter), tp, LOCATION_ONFIELD + LOCATION_HAND, 0, NO_EX);
    // if #sg<=0 then return end
    if sg.len() == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,#sg*4+3,sg):GetFirst()
    let lv = sg.len() as u32 * 4 + 3;
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv, &sg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &sel) else {
        return;
    };
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
    // local ct=tc:GetLevel()//4
    let ct = (card::get_level(d, tc) / 4) as usize;
    // local ssg=aux.SelectUnselectGroup(sg,e,tp,ct,ct,s.rescon(tc),1,tp,HINTMSG_TOGRAVE)
    let fusc = tc;
    let rescon_fn = move |d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| rescon(d, sg, e, tp, fusc);
    let ssg = aux::select_unselect_group(d, &sg, Some(e), tp, Some(ct), Some(ct), Some(&rescon_fn), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #ssg==0 then return end
    if ssg.len() == 0 {
        return;
    }
    // local fdg=ssg:Filter(aux.AND(Card.IsFacedown,Card.IsOnField),nil)
    let fdg = group::filter(d, &ssg, &|d, c| card::is_facedown(d, c) && card::is_on_field(d, c), NO_EX);
    // if #fdg>0 then
    if fdg.len() > 0 {
        // Duel.ConfirmCards(1-tp,fdg)
        duel::confirm_cards(d, 1 - tp, fdg);
    }
    // if Duel.SendtoGrave(ssg,REASON_EFFECT)>0 and ssg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) then
    if duel::send_to_grave(d, ssg.clone(), REASON_EFFECT, None, None) > 0
        && group::is_exists(d, &ssg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX)
    {
        // tc:SetMaterial(nil)
        card::set_material(d, tc, CardSet::new());
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummon(tc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)==0 then return end
        if duel::special_summon(d, tc, SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_AZAMINA) and c:IsMonster() and c:IsFaceup() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_AZAMINA as u16]) && card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE + LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand()
    //     and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_MZONE + LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_MZONE + LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_DECK + LOCATION_EXTRA)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
