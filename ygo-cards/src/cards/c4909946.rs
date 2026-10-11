//! Boss on Parade
use super::prelude::*;

pub const ID: u32 = 4909946;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[66947414], listed_series: &[SET_BES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: Add 1 "B.E.S." monster from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster in your hand or field, and if you do, Special Summon 1 LIGHT Machine monster with 1200 ATK and 1000 or less DEF from your Deck to either field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 1 "Boss Rush" from your Deck or GY face-up on your field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BES) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BES as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
// function s.desfilter(c,e,tp)
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetMZoneCount(tp,c)>0,Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0)
    if !card::is_monster(d, c) {
        return false;
    }
    let mmz_chk = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0;
    let opp_mmz_chk = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, mmz_chk, opp_mmz_chk)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,mmz_chk,opp_mmz_chk)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool, opp_mmz_chk: bool) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsAttack(1200) and c:IsDefenseBelow(1000)
    //     and ((mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    //     or (opp_mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)))
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_attack(d, c, &[1200])
        && card::is_defense_below(d, c, 1000)
        && ((mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
            || (opp_mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,e,tp)
    let dg = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #dg>0 and Duel.Destroy(dg,REASON_EFFECT)>0 then
    if !dg.is_empty() && duel::destroy(d, &dg, REASON_EFFECT, None, None) > 0 {
        // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // local opp_mmz_chk=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
        let opp_mmz_chk = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_chk,opp_mmz_chk):GetFirst()
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, mmz_chk, opp_mmz_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let sc = match group::get_first(d, &sg) {
            Some(sc) => sc,
            None => return,
        };
        // local b1=mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b1 = mmz_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None);
        // local b2=opp_mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
        let b2 = opp_mmz_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
        // if not (b1 or b2) then return end
        if !(b1 || b2) {
            return;
        }
        // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
        let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
        // local to_player=op==1 and tp or 1-tp
        let to_player = if op == Some(1) { tp } else { 1 - tp };
        // Duel.SpecialSummon(sc,0,tp,to_player,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, to_player, false, false, POS_FACEUP as u8, None);
    }
}
// function s.plfilter(c,tp)
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(66947414) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_code(d, c, &[66947414]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let nf = aux::necro_valley_filter(|d, c| plfilter(d, c, tp));
    let sg = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if sc then Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) end
    if let Some(sc) = group::get_first(d, &sg) {
        duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
