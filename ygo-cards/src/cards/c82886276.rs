//! Cynet Circuit
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 82886276;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIREWALL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon monsters from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return a "Firewall" monster to the Extra Deck and Special Summon it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(_,tp) return Duel.GetLP(tp)<=2000 end)
    effect::set_condition(d, e2, Some(lua_fn(|d, a| duel::get_lp(d, a.p(1)) <= 2000)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FIREWALL) and c:IsLinkMonster()
    //     and (Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c,c:GetLinkedZone(tp),tp)
    //     or Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c,c:GetLinkedZone(1-tp),1-tp))
    if !(card::is_set_card(d, c, &[SET_FIREWALL as u16]) && card::is_link_monster(d, c)) {
        return false;
    }
    let zone_tp = card::get_linked_zone(d, c, Some(tp));
    if duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, c, zone_tp, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        return true;
    }
    let zone_opp = card::get_linked_zone(d, c, Some(1 - tp));
    duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, c, zone_opp, 1 - tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,summonPlayer,targetCard,targetCardZones,toFieldPlayer)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, summon_player: u8, target_card: CardId, target_card_zones: u32, to_field_player: u8) -> bool {
    // local zone=(targetCardZones|(c:IsLinkMonster() and targetCard:GetToBeLinkedZone(c,toFieldPlayer) or 0))&ZONES_MMZ
    let link_zone = if card::is_link_monster(d, c) {
        card::get_to_be_linked_zone(d, target_card, c, to_field_player, false, false)
    } else {
        0
    };
    let zone = (target_card_zones | link_zone) & ZONES_MMZ;
    // return zone>0 and c:IsCanBeSpecialSummoned(e,0,summonPlayer,false,false,POS_FACEUP,toFieldPlayer,zone)
    zone > 0 && card::is_can_be_special_summoned(d, c, e, 0, summon_player, false, false, Some(POS_FACEUP as u8), Some(to_field_player), Some(zone))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        // A number is always true in Lua: the location count only has to be evaluated.
        let _ = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let _g = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let tp = a.tp;
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp,tc,tc:GetLinkedZone(tp),tp)
    let zone_tp = card::get_linked_zone(d, tc, Some(tp));
    let mut g1 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, tc, zone_tp, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp,tc,tc:GetLinkedZone(1-tp),1-tp)
    let zone_opp = card::get_linked_zone(d, tc, Some(1 - tp));
    let mut g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, tc, zone_opp, 1 - tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // while #(g1+g2)>0 do
    loop {
        let mut union = g1.clone();
        group::add_card(&mut union, &g2);
        if union.is_empty() {
            break;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=(g1+g2):Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &union, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // local b1=g1:IsContains(sc)
        let b1 = g1.contains(sc);
        // local b2=g2:IsContains(sc)
        let b2 = g2.contains(sc);
        // local op=Duel.SelectEffect(tp,
        //     {b1,aux.Stringid(id,2)},
        //     {b2,aux.Stringid(id,3)})
        let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
        // local toFieldPlayer=op==1 and tp or 1-tp
        let to_field_player = if op == Some(1) { tp } else { 1 - tp };
        // local zone=(tc:GetLinkedZone(toFieldPlayer)|(sc:IsLinkMonster() and tc:GetToBeLinkedZone(sc,toFieldPlayer) or 0))&ZONES_MMZ
        let tc_zone = card::get_linked_zone(d, tc, Some(to_field_player));
        let link_zone = if card::is_link_monster(d, sc) {
            card::get_to_be_linked_zone(d, tc, sc, to_field_player, false, false)
        } else {
            0
        };
        let zone = (tc_zone | link_zone) & ZONES_MMZ;
        // Duel.SpecialSummonStep(sc,0,tp,toFieldPlayer,false,false,POS_FACEUP,zone)
        duel::special_summon_step(d, sc, 0, tp, to_field_player, false, false, POS_FACEUP as u8, Some(zone));
        // g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp,tc,tc:GetLinkedZone(tp),tp)
        let zone_tp = card::get_linked_zone(d, tc, Some(tp));
        g1 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, tc, zone_tp, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
        // g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp,tc,tc:GetLinkedZone(1-tp),1-tp)
        let zone_opp = card::get_linked_zone(d, tc, Some(1 - tp));
        g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, tc, zone_opp, 1 - tp)), tp, LOCATION_GRAVE, 0, NO_EX);
        // if #(g1+g2)==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,4)) then break end
        let mut union = g1.clone();
        group::add_card(&mut union, &g2);
        if union.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
            break;
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FIREWALL) and c:IsLinkMonster() and c:IsAbleToExtra()
    card::is_set_card(d, c, &[SET_FIREWALL as u16]) && card::is_link_monster(d, c) && card::is_able_to_extra(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g,true)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        // local sc=Duel.GetOperatedGroup():GetFirst()
        let operated = duel::get_operated_group(d);
        let sc = group::get_first(d, &operated).nil();
        // if sc:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,sc)>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
        if card::is_location(d, sc, LOCATION_EXTRA)
            && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(sc), None).0 > 0
            && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 5))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
