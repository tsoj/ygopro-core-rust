//! Noh-P.U.N.K. Rising Scale
use super::prelude::*;

pub const ID: u32 = 75046994;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to your hand, or Special Summon, 1 "P.U.N.K." monster from your Deck or GY, except a Level 8 monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.PayLP(600))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(600))));
    // e2:SetTarget(s.thsptg)
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    // e2:SetOperation(s.thspop)
    effect::set_operation(d, e2, Some(operation(thspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change an opponent's monster with 2500 or more ATK that activates its effect to face-down Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_POSITION + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.poscon)
    effect::set_condition(d, e3, Some(condition(poscon)));
    // e3:SetTarget(s.postg)
    effect::set_target(d, e3, Some(cost_target(postg)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PUNK) and c:IsAbleToRemoveAsCost() and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_set_card(d, c, &[SET_PUNK as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,1,1,c)
    let tc = duel::select_matching_card(d, a.tp, Some(&spcostfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_COST)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thspfilter(c,e,tp,sp_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sp_chk: bool) -> bool {
    // return c:IsSetCard(SET_PUNK) and c:IsMonster() and not c:IsLevel(8)
    //     and (c:IsAbleToHand() or (sp_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_PUNK as u16])
        && card::is_monster(d, c)
        && !card::is_level(d, c, &[8])
        && (card::is_able_to_hand(d, c, None) || (sp_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local sp_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let sp_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,sp_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, sp_chk)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local sp_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let sp_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thspfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,sp_chk):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| thspfilter(d, c, a.e, a.tp, sp_chk))),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // aux.ToHandOrElse(tc,tp,
    //     function() return sp_chk and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function() Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3))
    let (e, tp) = (a.e, a.tp);
    aux::to_hand_or_else(
        d,
        tc,
        tp,
        Some(&move |d: &mut Duel, _x: CardId| sp_chk && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)),
        Some(&move |d: &mut Duel, _sc: &CardSet| duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
}
/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc,atk=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_ATTACK)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_ATTACK]);
    let loc = info[0].int();
    // return loc==LOCATION_MZONE and re:IsMonsterEffect() and re:GetHandler():IsControler(1-tp) and atk>=2500
    if loc != LOCATION_MZONE as i64 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    card::is_controler(d, rc, 1 - a.tp) && info[1].int() >= 2500
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if chk==0 then return rc:IsFaceup() and rc:IsCanTurnSet() and rc:IsRelateToEffect(re) end
    if a.chk == Some(0) {
        return card::is_faceup(d, rc) && card::is_can_turn_set(d, rc) && card::is_relate_to_effect(d, rc, re);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,rc,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(rc)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsRelateToEffect(re) then
    if card::is_relate_to_effect(d, rc, re) {
        // Duel.ChangePosition(rc,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, rc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
