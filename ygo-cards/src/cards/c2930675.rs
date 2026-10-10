//! World Legacy Key
use super::prelude::*;

pub const ID: u32 = 2930675;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKK_KNIGHT, SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate Trap effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.DoubleSnareValidity(c,LOCATION_SZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_MEKK_KNIGHT) and c:IsMonster()) or c:IsSetCard(SET_WORLD_LEGACY)) and c:IsFaceup() and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_monster(d, c)) || card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16]))
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetOperation(s.activate)
        effect::set_operation(d, a.e, Some(operation(activate)));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
/// `s.cfilter(c,seq,p)`
fn cfilter(d: &mut Duel, c: CardId, seq: u32, p: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MEKK_KNIGHT) and c:IsColumn(seq,p,LOCATION_SZONE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_column(d, c, seq, Some(p), Some(LOCATION_SZONE))
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not re:IsTrapEffect() then return false end
    let re = a.re.nil();
    if a.rp == a.tp || !effect::is_trap_effect(d, re) {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // local p,loc,seq=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_SEQUENCE)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SEQUENCE]);
    let p = info[0].int() as u8;
    let mut loc = info[1].int() as u32;
    let mut seq = info[2].int() as u32;
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and (loc&LOCATION_SZONE==0 or rc:IsControler(1-p)) then
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && (loc & LOCATION_SZONE == 0 || card::is_controler(d, rc, 1 - p)) {
        // if rc:IsLocation(LOCATION_SZONE) and rc:IsControler(p) then
        if card::is_location(d, rc, LOCATION_SZONE) && card::is_controler(d, rc, p) {
            // seq=rc:GetSequence()
            seq = card::get_sequence(d, rc);
            // loc=LOCATION_SZONE
            loc = LOCATION_SZONE;
        } else {
            // seq=rc:GetPreviousSequence()
            seq = card::get_previous_sequence(d, rc);
            // loc=rc:GetPreviousLocation()
            loc = card::get_previous_location(d, rc);
        }
    }
    // return loc&LOCATION_SZONE==LOCATION_SZONE and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,seq,p)
    loc & LOCATION_SZONE == LOCATION_SZONE && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, seq, p)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.disop(e,tp,...)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
