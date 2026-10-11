//! Seventh Eternity
//! Scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 60158866;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate or detach
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}

/// `s.nmbrfilter(c)`: `c:IsSetCard(SET_NUMBER) and no and no>=101 and no<=107`
fn nmbrfilter(d: &mut Duel, c: CardId) -> bool {
    let no = crate::card_info(d, c).xyz_number;
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && no.is_some_and(|no| (101..=107).contains(&no))
}

/// `c:GetOverlayGroup():IsExists(s.nmbrfilter,1,nil)`
fn overlay_has_nmbr(d: &mut Duel, c: CardId) -> bool {
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| nmbrfilter(d, x), 1, NO_EX)
}

/// `s.disfilter(c,atk)`: `c:IsNegatableMonster() and c:IsAttackBelow(atk)`
fn disfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    card::is_negatable_monster(d, c) && card::is_attack_below(d, c, atk)
}

/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let ovct = card::get_overlay_count(d, c);
    let atk = card::get_attack(d, c);
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && (nmbrfilter(d, c) || overlay_has_nmbr(d, c))
        && (duel::is_existing_matching_card(d, Some(&|d, x| disfilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            || (ovct > 0 && card::check_remove_overlay_card(d, c, tp, ovct as u16, REASON_EFFECT)))
}

/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if not (chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup()
        //     and (s.nmbrfilter(chkc) or chkc:GetOverlayGroup():IsExists(s.nmbrfilter,1,nil))) then return false end
        if !(card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && card::is_faceup(d, chkc)
            && (nmbrfilter(d, chkc) || overlay_has_nmbr(d, chkc)))
        {
            return false;
        }
        // local b1_rdr=Duel.IsExistingMatchingCard(s.disfilter,tp,0,LOCATION_MZONE,1,nil,chkc:GetAttack())
        let chkc_atk = card::get_attack(d, chkc);
        let b1_rdr = duel::is_existing_matching_card(d, Some(&|d, x| disfilter(d, x, chkc_atk)), tp, 0, LOCATION_MZONE, 1, NO_EX);
        // local ovct=chkc:GetOverlayCount()
        let ovct = card::get_overlay_count(d, chkc);
        // local b2_rdr=ovct>0 and chkc:CheckRemoveOverlayCard(tp,ovct,REASON_EFFECT)
        let b2_rdr = ovct > 0 && card::check_remove_overlay_card(d, chkc, tp, ovct as u16, REASON_EFFECT);
        // local op_rdr=e:GetLabel()
        let op_rdr = effect::get_label(d, a.e);
        // return (b1_rdr and op_rdr==1) or (b2_rdr and op_rdr==2)
        return (b1_rdr && op_rdr == 1) || (b2_rdr && op_rdr == 2);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local b1=Duel.IsExistingMatchingCard(s.disfilter,tp,0,LOCATION_MZONE,1,nil,tc:GetAttack())
    let tc_atk = card::get_attack(d, tc);
    let b1 = duel::is_existing_matching_card(d, Some(&|d, x| disfilter(d, x, tc_atk)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=tc:GetOverlayCount()>0 and tc:CheckRemoveOverlayCard(tp,tc:GetOverlayCount(),REASON_EFFECT)
    let tc_ovct = card::get_overlay_count(d, tc);
    let b2 = tc_ovct > 0 && card::check_remove_overlay_card(d, tc, tp, tc_ovct as u16, REASON_EFFECT);
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 0, 0);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}

/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 and tc:IsFaceup() then
    if op == 1 && card::is_faceup(d, tc) {
        // Negate the effects of 1 opponent's monster
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        // local ngc=Duel.SelectMatchingCard(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil,tc:GetAttack()):GetFirst()
        let tc_atk = card::get_attack(d, tc);
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| disfilter(d, x, tc_atk)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let Some(ngc) = group::get_first(d, &sel) else { return };
        // Duel.HintSelection(ngc,true)
        duel::hint_selection(d, ngc, true);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Duel.NegateRelatedChain(ngc,RESET_TURN_SET)
        duel::negate_related_chain(d, ngc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // ngc:RegisterEffect(e1)
        card::register_effect(d, ngc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // ngc:RegisterEffect(e2)
        card::register_effect(d, ngc, e2, false);
    // elseif op==2 then
    } else if op == 2 {
        // Detach all materials from the target
        // local og=tc:GetOverlayGroup()
        let og = card::get_overlay_group(d, tc);
        // if #og==0 then return end
        if og.is_empty() {
            return;
        }
        // if Duel.SendtoGrave(og,REASON_EFFECT)~=#og or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        let og_len = og.len() as i32;
        if duel::send_to_grave(d, og, REASON_EFFECT, None, None) != og_len
            || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        {
            return;
        }
        // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
        let f = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp));
        let sg = duel::get_matching_group(d, Some(&f), tp, LOCATION_GRAVE, 0, NO_EX);
        // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sc=sg:Select(tp,1,1,nil)
            let sc = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            // if #sc==0 then return end
            if sc.is_empty() {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
