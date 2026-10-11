//! Mariamne, the True Dracophoenix
use super::prelude::*;

pub const ID: u32 = 94160895;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Add 1 non-WIND Wyrm monster to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    card::is_monster(d, c) && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WIND)
    //     and (Duel.GetMZoneCount(tp)>0 or sg:IsExists(Card.IsInMainMZone,1,nil,tp))
    let wind = group::is_exists(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_WIND), 1, NO_EX);
    let ok = wind
        && (duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 > 0 || group::is_exists(d, sg, &|d, c| card::is_in_main_m_zone(d, c, Some(tp)), 1, NO_EX));
    (ok, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local loc=Duel.IsPlayerAffectedByEffect(tp,88581108) and LOCATION_MZONE or 0
    let loc = if duel::is_player_affected_by_effect(d, a.tp, 88581108).is_some() { LOCATION_MZONE } else { 0 };
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,loc,c)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, loc, c.into());
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // if #g==2 and g:FilterCount(Card.IsLocation,nil,LOCATION_HAND)<=1 then
    if g.len() == 2 && group::filter_count(d, &g, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX) <= 1 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,2,tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 2, a.tp, LOCATION_MZONE as i32);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local loc=Duel.IsPlayerAffectedByEffect(tp,88581108) and LOCATION_MZONE or 0
    let loc = if duel::is_player_affected_by_effect(d, a.tp, 88581108).is_some() { LOCATION_MZONE } else { 0 };
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,loc,c)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, loc, c.into());
    // if #g<2 or not g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WIND) then return end
    if g.len() < 2 || !group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WIND), 1, NO_EX) {
        return;
    }
    // local dg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_DESTROY)
    let dg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_DESTROY as u64), None, None, false)
        .unwrap_or_default();
    // local bnsh=dg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_WIND)==2
    let bnsh = group::filter_count(d, &dg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WIND), NO_EX) == 2;
    // if Duel.Destroy(dg,REASON_EFFECT)==2 then
    if duel::destroy(d, dg, REASON_EFFECT, None, None) == 2 {
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
        if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // local rg=Duel.GetDecktopGroup(1-tp,4)
        let rg = duel::get_decktop_group(d, 1 - a.tp, 4);
        // if bnsh and #rg>0 and rg:FilterCount(Card.IsAbleToRemove,nil)==4
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if bnsh
            && !rg.is_empty()
            && group::filter_count(d, &rg, &|d, x| card::is_able_to_remove(d, x, None, None, None), NO_EX) == 4
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.DisableShuffleCheck()
            duel::disable_shuffle_check(d, true);
            // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttributeExcept(ATTRIBUTE_WIND) and c:IsRace(RACE_WYRM) and c:IsAbleToHand()
    card::is_attribute_except(d, c, ATTRIBUTE_WIND, None, 0, None) && card::is_race(d, c, RACE_WYRM) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `function(e) return e:GetHandler():IsReason(REASON_EFFECT) end`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT)
}
