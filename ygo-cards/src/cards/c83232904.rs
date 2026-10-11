//! Clown Crew Malabarisme
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 83232904;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon up to 2 "Clown Crew" monsters from your Deck and/or Extra Deck, ignoring their Summoning conditions, also you cannot activate the effects of monsters Special Summoned from the Deck or Extra Deck until the end of the next turn after this card resolves
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase, if this card in your GY, except the turn it was sent there: You can banish it and Tribute 1 monster from your hand or field, then target 1 card on the field; return it to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.AND(Cost.SelfBanish,s.rthcost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::self_banish(), cost_target(rthcost)])));
    // e2:SetTarget(s.rthtg)
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    // e2:SetOperation(s.rthop)
    effect::set_operation(d, e2, Some(operation(rthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.spfilter(c,e,tp,mmz_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not (c:IsSetCard(SET_CLOWN_CREW) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_CLOWN_CREW as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return mmz_chk
        mmz_chk
    // elseif c:IsLocation(LOCATION_EXTRA) then
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // (the Lua function falls through and returns nil)
        false
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        let mmz = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.pendlinkfilter(c)`
fn pendlinkfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_EXTRA) and c:IsFaceup()) or c:IsLinkMonster()
    (card::is_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c)) || card::is_link_monster(d, c)
}
/// `s.rescon(mmz_ct,linkmz_ct)`: the returned function(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, mmz_ct: usize, linkmz_ct: usize) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=mmz_ct
    //     and sg:FilterCount(s.pendlinkfilter,nil)<=linkmz_ct
    group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= mmz_ct
        && group::filter_count(d, sg, &pendlinkfilter, NO_EX) <= linkmz_ct
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // You cannot activate the effects of monsters Special Summoned from the Deck or Extra Deck until the end of the next turn after this card resolves
        // local e1=Effect.CreateEffect(e:GetHandler())
        let owner = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, owner);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    // local mmz_ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mmz_ct = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local emz_ct=Duel.GetLocationCountFromEx(tp,tp,nil,nil,ZONES_EMZ)
    let emz_ct = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::None, Some(ZONES_EMZ)).0;
    // local linkmz_ct=Duel.GetLocationCountFromEx(tp,tp)
    let linkmz_ct = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::None, None).0;
    // local ft=math.min(mmz_ct+emz_ct,2)
    let mut ft = (mmz_ct + emz_ct).min(2);
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,nil,e,tp,mmz_ct>0)
    let mmz = mmz_ct > 0;
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon(mmz_ct,linkmz_ct),1,tp,HINTMSG_SPSUMMON)
    let (mmz_u, linkmz_u) = (mmz_ct.max(0) as usize, linkmz_ct.max(0) as usize);
    let rc = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        (rescon(d, sg, mmz_u, linkmz_u), false)
    };
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&rc), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local fup,fdown=sg:Split(s.pendlinkfilter,nil)
    let (fup, fdown) = group::split(d, &sg, &pendlinkfilter, NO_EX);
    // local fdown_main,fdown_ex=fdown:Split(Card.IsLocation,nil,LOCATION_DECK)
    let (fdown_main, fdown_ex) = group::split(d, &fdown, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // local priority_0,priority_1
    // if linkmz_ct<mmz_ct then
    let (priority_0, priority_1) = if linkmz_ct < mmz_ct {
        // priority_0=fup
        // priority_1=fdown_main
        (fup, fdown_main)
    } else {
        // priority_0=fdown_main
        // priority_1=fup
        (fdown_main, fup)
    };
    // for prio0_c in priority_0:Iter() do
    for prio0_c in group::iter(d, &priority_0) {
        // Duel.SpecialSummonStep(prio0_c,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, prio0_c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
    // for prio1_c in priority_1:Iter() do
    for prio1_c in group::iter(d, &priority_1) {
        // Duel.SpecialSummonStep(prio1_c,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, prio1_c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
    // for fdown_ex_c in fdown_ex:Iter() do
    for fdown_ex_c in group::iter(d, &fdown_ex) {
        // Duel.SpecialSummonStep(fdown_ex_c,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, fdown_ex_c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:IsOnField() and rc:IsSummonLocation(LOCATION_DECK|LOCATION_EXTRA)
    effect::is_monster_effect(d, re) && card::is_on_field(d, rc) && card::is_summon_location(d, rc, LOCATION_DECK | LOCATION_EXTRA)
}
/// `s.rthcostfilter(c)`
fn rthcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return Duel.IsExistingTarget(Card.IsAbleToHand,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
    duel::is_existing_target(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into())
}
/// `s.rthcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rthcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.rthcostfilter,1,true,nil,nil) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&rthcostfilter), 1, 1, true, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.rthcostfilter,1,1,true,nil,nil)
    let g = aux::select_release_group_cost(d, a.tp, Some(&rthcostfilter), 1, 1, true, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
