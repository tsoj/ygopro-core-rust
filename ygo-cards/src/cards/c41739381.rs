//! Clockwork Knight
//! 機械仕掛けの騎士
use super::prelude::*;

pub const ID: u32 = 41739381;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[84797028], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 non-Link Machine monster with 1000 or less original ATK
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 1, Some(1), None, None, None);
    // Add 1 "Clockwork Night" from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_link_summoned(d, h)
        })),
    );
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Machine monster with 1000 or less ATK from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,stype,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return not c:IsType(TYPE_LINK,lc,stype,tp) and c:IsRace(RACE_MACHINE,lc,stype,tp) and c:GetBaseAttack()<=1000
    !card::is_type_ctx(d, c, TYPE_LINK, sc, st, Some(tp)) && card::is_race_ctx(d, c, RACE_MACHINE, sc, st, Some(tp)) && card::get_base_attack(d, c) <= 1000
}
/// `s.thcostfilter(c)`
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsContinuousSpell() and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::is_continuous_spell(d, c) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thcostfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thcostfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(84797028) and c:IsAbleToHand()
    card::is_code(d, c, &[84797028]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.spfilter(c,e,tp,hc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: CardId) -> bool {
    // return c:IsAttackBelow(1000) and c:IsRace(RACE_MACHINE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //     and Duel.CheckReleaseGroup(tp,s.sptributefilter,1,false,1,true,c,tp,nil,false,hc)
    card::is_attack_below(d, c, 1000)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && duel::check_release_group(
            d,
            tp,
            Some(&sptributefilter),
            1,
            duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
            hc.into(),
            false,
        )
}
/// `s.sptributefilter(c)`
fn sptributefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsReleasableByEffect()
    card::is_race(d, c, RACE_MACHINE) && card::is_releasable_by_effect(d, c, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp, c);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| spfilter(d, cc, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,c)
    let g = duel::select_target(d, tp, Some(&|d, cc| spfilter(d, cc, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local g=Duel.SelectReleaseGroup(tp,s.sptributefilter,1,1,false,false,true,tc,tp,nil,false,exc)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&sptributefilter),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: false, check_field: true, to_check: tc, toplayer: Some(tp), use_oppo: false, ..Default::default() },
        exc.map_or(NO_EX, |x| x.into()),
        false,
    )
    .nil();
    // if #g>0 and Duel.Release(g,REASON_EFFECT)>0 and tc:IsRelateToEffect(e) then
    if !g.is_empty() && duel::release(d, g, REASON_EFFECT, None) > 0 {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
