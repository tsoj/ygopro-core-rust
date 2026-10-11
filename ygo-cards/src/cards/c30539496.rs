//! True King Lithosagym, the Disaster
use super::prelude::*;

pub const ID: u32 = 30539496;
/// `CARD_TRUE_KING_CALAMITIES`
const CARD_TRUE_KING_CALAMITIES: u32 = 88581108;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
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
    // Special summon a monster from the graveyard
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop2)
    effect::set_operation(d, e2, Some(operation(spop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and ((c:IsLocation(LOCATION_MZONE) and c:IsFaceup()) or c:IsLocation(LOCATION_HAND))
    card::is_monster(d, c)
        && ((card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)) || card::is_location(d, c, LOCATION_HAND))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_EARTH) and Duel.GetMZoneCount(tp,sg)>0
    let ok = group::is_exists(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_EARTH), 1, NO_EX)
        && duel::get_m_zone_count(d, tp, sg.into(), Some(tp), None, None).0 > 0;
    (ok, false)
}
/// The `(desfilter)` group of `sptg` / `spop`: `Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,opp_mzone,c)`
fn desgroup(d: &mut Duel, tp: u8, c: CardId) -> CardSet {
    // local opp_mzone=Duel.IsPlayerAffectedByEffect(tp,CARD_TRUE_KING_CALAMITIES) and LOCATION_MZONE or 0
    let opp_mzone = if duel::is_player_affected_by_effect(d, tp, CARD_TRUE_KING_CALAMITIES).is_some() { LOCATION_MZONE } else { 0 };
    duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE | LOCATION_HAND, opp_mzone, c.into())
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=desgroup
    let g = desgroup(d, tp, c);
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, cc: Option<CardId>| rescon(d, sg, e, tp, mg, cc)), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // if (#g==2 and g:FilterCount(Card.IsLocation,nil,LOCATION_HAND)==1) or not g:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then
    let hand_count = group::filter_count(d, &g, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
    if (g.len() == 2 && hand_count == 1) || !group::is_exists(d, &g, &|d, x| card::is_location(d, x, LOCATION_HAND), 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, tp, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,2,tp,LOCATION_MZONE|LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 2, tp, (LOCATION_MZONE | LOCATION_HAND) as i32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=desgroup
    let g = desgroup(d, tp, c);
    // if #g<2 or not g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_EARTH) then return end
    if g.len() < 2 || !group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_EARTH), 1, NO_EX) {
        return;
    }
    // local dg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_DESTROY)
    let dg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(2),
        Some(2),
        Some(&|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, cc: Option<CardId>| rescon(d, sg, e, tp, mg, cc)),
        1,
        tp,
        Some(HINTMSG_DESTROY as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #dg<2 then return end
    if dg.len() < 2 {
        return;
    }
    // local rmv_chk=dg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_EARTH)==2 and true or false
    let rmv_chk = group::filter_count(d, &dg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_EARTH), NO_EX) == 2;
    // if Duel.Destroy(dg,REASON_EFFECT)==2 and c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 and rmv_chk then
    if duel::destroy(d, dg, REASON_EFFECT, None, None) == 2
        && card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && rmv_chk
    {
        // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,nil)
        let rg = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, 0, LOCATION_EXTRA, NO_EX);
        // if rmv_chk and #rg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if rmv_chk && !rg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.ConfirmCards(tp,rg)
            duel::confirm_cards(d, tp, rg.clone());
            // local rmv_g=aux.SelectUnselectGroup(rg,e,tp,1,3,aux.dncheck,1,tp,HINTMSG_REMOVE)
            let dn = aux::dncheck();
            let rmv_g = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(1), Some(3), Some(&dn), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
                .unwrap_or_default();
            // if #rmv_g>0 then
            if !rmv_g.is_empty() {
                // Duel.Remove(rmv_g,POS_FACEUP,REASON_EFFECT)
                duel::remove(d, rmv_g, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
            // Duel.ShuffleExtra(1-tp)
            duel::shuffle_extra(d, 1 - tp);
        }
    }
}
/// `s.thfilter(c,e,tp)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttributeExcept(ATTRIBUTE_EARTH) and c:IsRace(RACE_WYRM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute_except(d, c, ATTRIBUTE_EARTH, None, 0, None)
        && card::is_race(d, c, RACE_WYRM)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e) return e:GetHandler():IsReason(REASON_EFFECT) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT)
}
