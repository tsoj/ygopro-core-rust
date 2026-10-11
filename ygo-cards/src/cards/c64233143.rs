//! Daedalus Maelstrom
use super::prelude::*;

pub const ID: u32 = 64233143;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON, CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent would Normal or Special Summon a monster(s) while you control "Atlantis, the Dragon City": Negate the Summon, and if you do, destroy that monster(s), then if you control a WATER Link Monster, you can destroy all non-WATER monsters on the field
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1a, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e1a:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1a, EFFECT_TYPE_ACTIVATE);
    // e1a:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1a, EVENT_SUMMON);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(s.condition)
    effect::set_condition(d, e1a, Some(condition(cond)));
    // e1a:SetTarget(s.target)
    effect::set_target(d, e1a, Some(cost_target(target)));
    // e1a:SetOperation(s.activate)
    effect::set_operation(d, e1a, Some(operation(activate)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1b, EVENT_SPSUMMON);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // You can banish this card from your GY and send 1 face-up "Umi" you control to the GY, then target 1 monster your opponent controls; send it to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.AND(Cost.SelfBanish,s.tgcost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::self_banish(), cost_target(tgcost)])));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.GetCurrentChain(true)==0 and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON),tp,LOCATION_ONFIELD,0,1,nil)
    let eg = a.eg.clone().nil();
    duel::get_current_chain(d, true) == 0
        && group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
        && duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON]))),
            tp,
            LOCATION_ONFIELD,
            0,
            1,
            NO_EX,
        )
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), eg.len() as u8, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg.clone()), eg.len() as u8, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
// function s.desconfilter(c)
fn desconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsLinkMonster()
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_link_monster(d, c)
}
// function s.desfilter(c)
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttributeExcept(ATTRIBUTE_WATER) or c:IsFacedown()
    card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None) || card::is_facedown(d, c)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // if Duel.Destroy(eg,REASON_EFFECT)>0 and Duel.IsExistingMatchingCard(s.desconfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
        && duel::is_existing_matching_card(d, Some(&desconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
// function s.tgcostfilter(c)
fn tgcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_UMI) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[CARD_UMI]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
// function s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgcostfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgcostfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgcostfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_grave(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, tp, 0);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoGrave(tc,REASON_EFFECT)
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
