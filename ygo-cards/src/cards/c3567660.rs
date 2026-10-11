//! Link Back
use super::prelude::*;

pub const ID: u32 = 3567660;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_TOGRAVE);
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
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster() and c:IsInExtraMZone()
    //     and c:GetLink()>0 and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,c:GetLinkedZone(tp)&ZONES_MMZ)>0
    if !(card::is_faceup(d, c) && card::is_link_monster(d, c) && card::is_in_extra_m_zone(d, c, None)) {
        return false;
    }
    if card::get_link(d, c) <= 0 {
        return false;
    }
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
    // local tc=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local ct=tc:GetLink()
    let ct = card::get_link(d, tc);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,ct)
    duel::set_possible_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, ct as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,Duel.GetDecktopGroup(tp,ct),ct,tp,LOCATION_DECK)
    let decktop = duel::get_decktop_group(d, tp, ct);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, Some(decktop), ct as u8, tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) or not tc:IsControler(tp) then return end
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_relate_to_effect(d, tc, a.e) || !card::is_controler(d, tc, tp) {
        return;
    }
    // local zone=tc:GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, tc, Some(tp)) & ZONES_MMZ;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,zone)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
        // local nseq=math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,LOCATION_MZONE,~zone),2)
        let field = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, LOCATION_MZONE, Some(!zone), false);
        if field == 0 {
            // Duel.SelectDisableField returned nil: math.log(nil,2) is an error in Lua
            crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
        }
        let nseq = lua_int((field as f64).log2()) as u8;
        // Duel.MoveSequence(tc,nseq)
        duel::move_sequence(d, tc, nseq, None);
        // local ct=tc:GetLink()
        let ct = card::get_link(d, tc);
        // if ct>0 and Duel.IsPlayerCanDiscardDeck(tp,ct) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if ct > 0 && duel::is_player_can_discard_deck(d, tp, ct) && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.DiscardDeck(tp,ct,REASON_EFFECT)
            duel::discard_deck(d, tp, ct as u16, REASON_EFFECT);
        }
    }
}
