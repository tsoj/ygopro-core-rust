//! Inferno of the Ashened
use super::prelude::*;

pub const ID: u32 = 62767644;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_OBSIDIM_ASHENED_CITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can place 1 "Obsidim, the Ashened City" from your Deck face-up in either Field Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 face-up card your opponent controls to the GY and Special Summon 1 Level 8 or higher DARK Pyro monster from your GY to your opponent's field in Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tgsptg)
    effect::set_target(d, e2, Some(cost_target(tgsptg)));
    // e2:SetOperation(s.tgspop)
    effect::set_operation(d, e2, Some(operation(tgspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_OBSIDIM_ASHENED_CITY) and not c:IsForbidden()
    card::is_code(d, c, &[CARD_OBSIDIM_ASHENED_CITY]) && !card::is_forbidden(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.plfilter,tp,LOCATION_DECK,0,nil,tp)
    let _g = duel::get_matching_group(d, Some(&plfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_DECK, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &g) {
            // local op=Duel.SelectOption(tp,false,aux.Stringid(id,3),aux.Stringid(id,4))
            let op = duel::select_option(d, tp, false, &[aux::stringid(ID, 3), aux::stringid(ID, 4)]);
            // local target_player=op==0 and tp or 1-tp
            let target_player = if op == 0 { tp } else { 1 - tp };
            // Duel.MoveToField(sc,tp,target_player,LOCATION_FZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, target_player, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsAbleToGrave() and Duel.GetMZoneCount(1-tp,c)>0
    card::is_faceup(d, c) && card::is_able_to_grave(d, c, None) && duel::get_m_zone_count(d, 1 - tp, c.into(), None, None, None).0 > 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(8) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_PYRO)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,1-tp)
    card::is_level_above(d, c, 8)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_PYRO)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), None)
}
/// `s.tgsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_ONFIELD,1,nil,tp)
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_ONFIELD,1,1,nil,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g2 = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g1,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g1, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g2,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g2, 1, tp, 0);
    true
}
/// `s.tgspop(e,tp,eg,ep,ev,re,r,rp)`
fn tgspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local fg,gg=tg:Split(Card.IsLocation,nil,LOCATION_ONFIELD)
    let (fg, gg) = group::split(d, &tg, &|d, c| card::is_location(d, c, LOCATION_ONFIELD), NO_EX);
    // if #fg>0 and Duel.SendtoGrave(fg,REASON_EFFECT)>0 and fg:GetFirst():IsLocation(LOCATION_GRAVE)
    //   and #gg>0 then
    if !fg.is_empty()
        && duel::send_to_grave(d, fg.clone(), REASON_EFFECT, None, None) > 0
        && card::is_location(d, group::get_first(d, &fg).nil(), LOCATION_GRAVE)
        && !gg.is_empty()
    {
        // Duel.SpecialSummon(gg,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, gg, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
