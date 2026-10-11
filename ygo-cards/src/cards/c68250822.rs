//! Spright Double Cross
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 68250822;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Target 1 monster on the field and GY and activate 1 effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
}
/// `s.atchfilter(c,tp)`
fn atchfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and not c:IsType(TYPE_TOKEN)
    //   and (c:IsControler(tp) or c:IsLocation(LOCATION_GRAVE) or c:IsAbleToChangeControler())
    //   and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,0,1,c)
    card::is_monster(d, c)
        && !card::is_type(d, c, TYPE_TOKEN)
        && (card::is_controler(d, c, tp) || card::is_location(d, c, LOCATION_GRAVE) || card::is_able_to_change_controler(d, c))
        && duel::is_existing_matching_card(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsRank(2) and c:IsFaceup()
    card::is_type(d, c, TYPE_XYZ) && card::is_rank(d, c, &[2]) && card::is_faceup(d, c)
}
/// `s.ctrlfilter(c,zone)`
fn ctrlfilter(d: &mut Duel, c: CardId, zone: u32) -> bool {
    // return c:IsControlerCanBeChanged(false,zone)
    card::is_controler_can_be_changed(d, c, false, Some(zone))
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `aux.GetMMZonesPointedTo(tp,Card.IsLink,LOCATION_MZONE,0,nil,2)`
fn link_zone(d: &mut Duel, tp: u8) -> u32 {
    aux::get_mm_zones_pointed_to(d, tp, Some(&|d, c| card::is_link(d, c, &[2])), Some(LOCATION_MZONE), Some(0), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=aux.GetMMZonesPointedTo(tp,Card.IsLink,LOCATION_MZONE,0,nil,2)
    let zone = link_zone(d, a.tp);
    // local b1=Duel.IsExistingTarget(s.atchfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil,tp)
    let b1 = duel::is_existing_target(d, Some(&|d, x| atchfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.ctrlfilter,tp,0,LOCATION_MZONE,1,nil,zone)
    let b2 = duel::is_existing_target(d, Some(&|d, x| ctrlfilter(d, x, zone)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b3=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp,zone)
    let b3 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local label=e:GetLabel()
        let label = effect::get_label(d, a.e);
        // if label==1 then
        if label == 1 {
            // return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.atchfilter(chkc,tp)
            return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && atchfilter(d, chkc, a.tp);
        // elseif label==2 then
        } else if label == 2 {
            // return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.ctrlfilter(chkc,zone)
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && ctrlfilter(d, chkc, zone);
        // elseif label==3 then
        } else if label == 3 {
            // return chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp,zone)
            return card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp, zone);
        }
    }
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,1)},
    //   {b2,aux.Stringid(id,2)},
    //   {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // local g=Duel.SelectTarget(tp,s.atchfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, x| atchfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // if g:GetFirst():IsLocation(LOCATION_GRAVE) then
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
            duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
        }
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, a.e, CATEGORY_CONTROL);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local g=Duel.SelectTarget(tp,ctrlfilter,tp,0,LOCATION_MZONE,1,1,nil,zone)
        // (the script's `ctrlfilter` is not `s.ctrlfilter`: a nil global, so no filter)
        let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    // else
    } else {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp,zone)
        let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, a.tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local zone=aux.GetMMZonesPointedTo(tp,Card.IsLink,LOCATION_MZONE,0,nil,2)
    let zone = link_zone(d, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then --Attach it to a Rank 2
    if op == 1 {
        // if not tc:IsImmuneToEffect(e) and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,0,1,tc) then
        if !card::is_immune_to_effect(d, tc, a.e) && duel::is_existing_matching_card(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, 1, tc.into()) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
            // local xc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,tc):GetFirst()
            let g = duel::select_matching_card(d, a.tp, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, tc.into()).nil();
            // if xc then
            if let Some(xc) = group::get_first(d, &g) {
                // Duel.Overlay(xc,tc,true)
                duel::overlay(d, xc, tc, true);
            }
        }
    // elseif op==2 then --Gain control of it to a zone a Link 2 points to
    } else if op == 2 {
        // if tc:IsRelateToEffect(e) and tc:IsControler(1-tp) then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp) {
            // Duel.GetControl(tc,tp,0,0,zone)
            duel::get_control(d, tc, a.tp, 0, 0, Some(zone), None);
        }
    // else --Special summon it to a zone a Link 2 points to
    } else {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and zone>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone) then
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && zone > 0
            && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone))
        {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,zone)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
        }
    }
}
