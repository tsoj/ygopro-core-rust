//! Xyz Force
use super::prelude::*;

pub const ID: u32 = 31398842;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_XYZ], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "Xyz" card to the GY or add it to the hand if an Xyz monster with an Xyz monster as material is on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach 1 material from an Xyz, then you can Special Summon it if it is an Xyz Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.detachtg)
    effect::set_target(d, e2, Some(cost_target(detachtg)));
    // e2:SetOperation(s.detachop)
    effect::set_operation(d, e2, Some(operation(detachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:GetOverlayGroup():IsExists(Card.IsType,1,nil,TYPE_XYZ)
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let mats = card::get_overlay_group(d, c);
    group::is_exists(d, &mats, &|d, c| card::is_type(d, c, TYPE_XYZ), 1, NO_EX)
}
/// `s.cfilter(c,to_hand)`
fn cfilter(d: &mut Duel, c: CardId, to_hand: bool) -> bool {
    // return c:IsSetCard(SET_XYZ) and not c:IsCode(id) and (c:IsAbleToGrave() or (to_hand and c:IsAbleToHand()))
    card::is_set_card(d, c, &[SET_XYZ as u16])
        && !card::is_code(d, c, &[ID])
        && (card::is_able_to_grave(d, c, None) || (to_hand && card::is_able_to_hand(d, c, None)))
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local to_hand=Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let to_hand = duel::is_existing_matching_card(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_DECK,0,1,nil,to_hand) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, to_hand)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local to_hand=Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let to_hand = duel::is_existing_matching_card(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // local hintmsg=to_hand and aux.Stringid(id,2) or HINTMSG_TOGRAVE
    let hintmsg = if to_hand { aux::stringid(ID, 2) } else { HINTMSG_TOGRAVE as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,hintmsg)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, hintmsg);
    // local tc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_DECK,0,1,1,nil,to_hand):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c, to_hand)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc then return end
    let Some(tc) = tc else { return };
    // if to_hand and tc:IsAbleToHand() then
    if to_hand && card::is_able_to_hand(d, tc, None) {
        // aux.ToHandOrElse(tc,tp)
        aux::to_hand_or_else(d, tc, a.tp, None, None, None);
    } else {
        // Duel.SendtoGrave(tc,REASON_EFFECT)
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.detachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn detachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckRemoveOverlayCard(tp,1,1,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, a.tp, 1, 1, 1, REASON_EFFECT, None);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_REMOVED | LOCATION_GRAVE) as i32);
    true
}
/// `s.detachop(e,tp,eg,ep,ev,re,r,rp)`
fn detachop(d: &mut Duel, a: &EffArgs) {
    // if Duel.RemoveOverlayCard(tp,1,1,1,1,REASON_EFFECT)==0 then return end
    if duel::remove_overlay_card(d, a.tp, 1, 1, 1, 1, REASON_EFFECT, None) == 0 {
        return;
    }
    // local sc=Duel.GetOperatedGroup():GetFirst()
    let og = duel::get_operated_group(d);
    let sc = group::get_first(d, &og);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and sc:IsControler(tp) and sc:IsMonster()
    //   and sc:IsType(TYPE_XYZ) and sc:IsLocation(LOCATION_REMOVED|LOCATION_GRAVE)
    //   and sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and aux.nvfilter(sc)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        let sc = sc.nil();
        if card::is_controler(d, sc, a.tp)
            && card::is_monster(d, sc)
            && card::is_type(d, sc, TYPE_XYZ)
            && card::is_location(d, sc, LOCATION_REMOVED | LOCATION_GRAVE)
            && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
            && aux::nvfilter(d, sc)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
