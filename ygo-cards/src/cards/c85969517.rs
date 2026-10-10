//! Plunder Patrollship Jord
use super::prelude::*;

pub const ID: u32 = 85969517;

/// `local TOKEN_PLUNDER_PATROLL=id+1`
const TOKEN_PLUNDER_PATROLL: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[SET_PLUNDER_PATROLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // 1 Tuner + 1+ Non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Return this card to the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_TOKEN+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_TOKEN + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 "Plunder Patroll" card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.dthcon)
    effect::set_condition(d, e2, Some(condition(dthcon)));
    // e2:SetTarget(s.dthtg)
    effect::set_target(d, e2, Some(cost_target(dthtg)));
    // e2:SetOperation(s.dthop)
    effect::set_operation(d, e2, Some(operation(dthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Plunder Patroll" card from the GY to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.gthtg)
    effect::set_target(d, e3, Some(cost_target(gthtg)));
    // e3:SetOperation(s.gthop)
    effect::set_operation(d, e3, Some(operation(gthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.attcheck(tp,att,targ_p)`
fn attcheck(d: &mut Duel, tp: u8, att: u32, targ_p: u8) -> bool {
    // return Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_PLUNDER_PATROLL,SET_PLUNDER_PATROLL,TYPES_TOKEN,0,0,4,RACE_FIEND,att,POS_FACEUP_DEFENSE,targ_p)
    let md = duel::MonsterData {
        setcodes: Some(vec![SET_PLUNDER_PATROLL as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(4),
        race: Some(RACE_FIEND),
        attribute: Some(att),
    };
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_PLUNDER_PATROLL, md, Some(POS_FACEUP_DEFENSE as u8), Some(targ_p), 0)
}
/// `s.getvalidatts(tp)`
fn getvalidatts(d: &mut Duel, tp: u8) -> u32 {
    // local res=ATTRIBUTE_ALL
    let mut res = ATTRIBUTE_ALL;
    // local att=ATTRIBUTE_EARTH
    let mut att = ATTRIBUTE_EARTH;
    // while att<ATTRIBUTE_ALL do
    while att < ATTRIBUTE_ALL {
        // if not s.attcheck(tp,att,tp) or not s.attcheck(tp,att,1-tp) then res=res&~att end
        if !attcheck(d, tp, att, tp) || !attcheck(d, tp, att, 1 - tp) {
            res &= !att;
        }
        // att=att<<1
        att <<= 1;
    }
    // return res
    res
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local valid_atts=s.getvalidatts(tp)
    let valid_atts = getvalidatts(d, a.tp);
    // if chk==0 then return valid_atts>0 and c:IsAbleToExtra()
    //     and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.GetMZoneCount(1-tp,c,tp)>0
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) end
    if a.chk == Some(0) {
        return valid_atts > 0
            && card::is_able_to_extra(d, c)
            && duel::get_m_zone_count(d, a.tp, c.into(), None, None, None).0 > 0
            && duel::get_m_zone_count(d, 1 - a.tp, c.into(), Some(a.tp), None, None).0 > 0
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none();
    }
    // local att=Duel.AnnounceAttribute(tp,1,valid_atts)
    let att = duel::announce_attribute(d, a.tp, 1, valid_atts);
    // Duel.SetTargetParam(att)
    duel::set_target_param(d, att);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // or not c:IsLocation(LOCATION_EXTRA) then return end
    if !card::is_location(d, c, LOCATION_EXTRA) {
        return;
    }
    // local att=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let att = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if not (att and att>0 and s.attcheck(tp,att,tp) and s.attcheck(tp,att,1-tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if att <= 0 {
        return;
    }
    let att = att as u32;
    if !(attcheck(d, a.tp, att, a.tp) && attcheck(d, a.tp, att, 1 - a.tp)) {
        return;
    }
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    if duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 <= 0 {
        return;
    }
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // for p=tp,1-tp,(tp==0 and 1 or -1) do
    let order: [u8; 2] = if a.tp == 0 { [0, 1] } else { [1, 0] };
    for p in order {
        // local token=Duel.CreateToken(tp,TOKEN_PLUNDER_PATROLL)
        let token = duel::create_token(d, a.tp, TOKEN_PLUNDER_PATROLL).nil();
        // token:Attribute(att)
        card::info_attribute(d, token, Some(att));
        // Duel.SpecialSummonStep(token,0,tp,p,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token, 0, a.tp, p, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.dthcon(e,tp,eg,ep,ev,re,r,rp)`
fn dthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PLUNDER_PATROLL) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.dthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_PLUNDER_PATROLL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.dthop(e,tp,eg,ep,ev,re,r,rp)`
fn dthop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 or not g:GetFirst():IsLocation(LOCATION_HAND) then return end
    if g.is_empty() {
        return;
    }
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    let first = group::get_first(d, &g).nil();
    if !card::is_location(d, first, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g);
    // if e:GetHandler():GetEquipGroup():IsExists(aux.FaceupFilter(Card.IsSetCard,SET_PLUNDER_PATROLL),1,nil)
    let h = effect::get_handler(d, a.e).nil();
    let eqg = card::get_equip_group(d, h);
    let equip_ok = group::is_exists(
        d,
        &eqg,
        &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]),
        1,
        NO_EX,
    );
    // and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    // and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    // and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if equip_ok
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.gthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    //     and e:GetHandler():IsType(TYPE_PENDULUM) and Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && card::is_type(d, h, TYPE_PENDULUM)
            && duel::check_pendulum_zones(d, a.tp);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.gthop(e,tp,eg,ep,ev,re,r,rp)`
fn gthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_HAND) and Duel.CheckPendulumZones(tp)
    //     and c:IsRelateToEffect(e) and not c:IsForbidden() then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
        && duel::check_pendulum_zones(d, a.tp)
        && card::is_relate_to_effect(d, c, a.e)
        && !card::is_forbidden(d, c)
    {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
