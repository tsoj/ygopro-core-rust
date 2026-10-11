//! Ogdoadic Serpent Strike
//! 溟界の大蛟
use super::prelude::*;

pub const ID: u32 = 96203584;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon a Reptile from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send a Reptile from Deck to GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.tgcon)
    effect::set_condition(d, e3, Some(condition(tgcon)));
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcfilter(c,e,tp)`
fn spcfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsAbleToGraveAsCost() and Duel.GetMZoneCount(tp,c)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c:GetOriginalAttribute())
    if !card::is_monster(d, c) || !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let att = card::get_original_attribute(d, c);
    duel::is_existing_target(d, Some(&|d, cc| spfilter(d, cc, e, tp, att)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,att)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32) -> bool {
    // return c:IsRace(RACE_REPTILE) and not c:IsOriginalAttribute(att) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_REPTILE)
        && !card::is_original_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spcfilter(d, c, e, tp)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spcfilter(d, c, e, tp)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local att=g:GetFirst():GetOriginalAttribute()
    let first = group::get_first(d, &g).nil();
    let att = card::get_original_attribute(d, first);
    // e:SetLabel(att)
    effect::set_label(d, e, &[att as i64]);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local attr=e:GetLabel()
    let attr = effect::get_label(d, e) as u32;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp,attr) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp, attr);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,attr)
    let sg = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp, attr)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, sg, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.tgcfilter(c,tp)`
fn tgcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsMonster()
    card::is_controler(d, c, 1 - tp) && card::is_monster(d, c)
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and eg:IsExists(s.tgcfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    duel::is_main_phase(d, None) && group::is_exists(d, &eg, &|d, c| tgcfilter(d, c, tp), 1, NO_EX)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_REPTILE) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_race(d, c, RACE_REPTILE) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
