//! Castle Link
use super::prelude::*;

pub const ID: u32 = 22198672;

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
    // move
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.seqtg)
    effect::set_target(d, e2, Some(cost_target(seqtg)));
    // e2:SetOperation(s.seqop)
    effect::set_operation(d, e2, Some(operation(seqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // change
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.chtg)
    effect::set_target(d, e3, Some(cost_target(chtg)));
    // e3:SetOperation(s.chop)
    effect::set_operation(d, e3, Some(operation(chop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsLinkMonster() then return false end
    if !card::is_link_monster(d, c) {
        return false;
    }
    // local p=c:GetControler()
    let p = card::get_controler(d, c);
    // local zone=c:GetLinkedZone()&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, None) & ZONES_MMZ;
    // return Duel.GetLocationCount(p,LOCATION_MZONE,p,LOCATION_REASON_CONTROL,zone)>0
    duel::get_location_count(d, p, LOCATION_MZONE as u8, Some(p), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local p=tc:GetControler()
    let p = card::get_controler(d, tc);
    // local zone=tc:GetLinkedZone()&ZONES_MMZ
    let zone = card::get_linked_zone(d, tc, None) & ZONES_MMZ;
    // if Duel.GetLocationCount(p,LOCATION_MZONE,p,LOCATION_REASON_CONTROL,zone)>0 then
    if duel::get_location_count(d, p, LOCATION_MZONE as u8, Some(p), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0 {
        // local i=0
        let mut i: u32 = 0;
        // if not tc:IsControler(tp) then i=16 end
        if !card::is_controler(d, tc, tp) {
            i = 16;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
        // local nseq=math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,LOCATION_MZONE,~(zone<<i)),2) - i
        let field = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, LOCATION_MZONE, Some(!(zone << i)), false);
        let nseq = (field as f64).log2() - i as f64;
        // Duel.MoveSequence(tc,nseq)
        duel::move_sequence(d, tc, nseq as u8, None);
    }
}
/// `s.chfilter1(c)`
fn chfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkMonster() and c:GetSequence()<5
    //     and Duel.IsExistingMatchingCard(s.chfilter2,c:GetControler(),LOCATION_MZONE,0,1,c)
    card::is_link_monster(d, c) && card::get_sequence(d, c) < 5 && {
        let p = card::get_controler(d, c);
        duel::is_existing_matching_card(d, Some(&chfilter2), p, LOCATION_MZONE, 0, 1, c.into())
    }
}
/// `s.chfilter2(c)`
fn chfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkMonster() and c:GetSequence()<5
    card::is_link_monster(d, c) && card::get_sequence(d, c) < 5
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.chfilter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&chfilter1), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.chfilter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g1 = duel::select_matching_card(d, tp, Some(&chfilter1), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local tc1=g1:GetFirst()
    // if not tc1 then return end
    let Some(tc1) = group::get_first(d, &g1) else { return };
    // Duel.HintSelection(g1)
    duel::hint_selection(d, g1.clone(), true);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.chfilter2,tc1:GetControler(),LOCATION_MZONE,0,1,1,tc1)
    let p1 = card::get_controler(d, tc1);
    let g2 = duel::select_matching_card(d, tp, Some(&chfilter2), p1, LOCATION_MZONE, 0, 1, 1, false, tc1.into()).nil();
    // Duel.HintSelection(g2)
    duel::hint_selection(d, g2.clone(), true);
    // local tc2=g2:GetFirst()
    let tc2 = group::get_first(d, &g2).nil();
    // Duel.SwapSequence(tc1,tc2)
    duel::swap_sequence(d, tc1, tc2);
}
