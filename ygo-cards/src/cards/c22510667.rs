//! Gouki The Solid Ogre
use super::prelude::*;

pub const ID: u32 = 22510667;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], listed_names: &[22510667], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GOUKI),2)
    link::add_procedure(
        d,
        c,
        Some(crate::procs::mat_filter(|d, cc, sc, st, tp| card::is_set_card_ctx(d, cc, &[SET_GOUKI as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(s.incon)
    effect::set_condition(d, e1, Some(condition(incon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // move
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.seqtg)
    effect::set_target(d, e3, Some(cost_target(seqtg)));
    // e3:SetOperation(s.seqop)
    effect::set_operation(d, e3, Some(operation(seqop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.incon(e)`
fn incon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetLinkedGroupCount()>0 and e:GetHandler():GetLinkedGroup():IsExists(aux.FaceupFilter(Card.IsSetCard,SET_GOUKI),1,nil)
    let h = effect::get_handler(d, a.e).nil();
    if card::get_linked_group_count(d, h) == 0 {
        return false;
    }
    let lg = card::get_linked_group(d, h);
    group::is_exists(d, &lg, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GOUKI as u16]), 1, NO_EX)
}
/// `s.seqfilter(c,zone)`
fn seqfilter(d: &mut Duel, c: CardId, zone: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GOUKI) and not c:IsCode(id) and c:GetSequence()<5 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE,c:GetControler(),LOCATION_REASON_CONTROL,zone)>0
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GOUKI as u16]) && !card::is_code(d, c, &[ID]) && card::get_sequence(d, c) < 5) {
        return false;
    }
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, Some(p), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=c:GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.seqfilter(chkc,zone) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && seqfilter(d, chkc, zone);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.seqfilter,tp,LOCATION_MZONE,0,1,nil,zone) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| seqfilter(d, cc, zone)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.seqfilter,tp,LOCATION_MZONE,0,1,1,nil,zone)
    duel::select_target(d, tp, Some(&|d, cc| seqfilter(d, cc, zone)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=c:GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) or Duel.GetLocationCount(tc:GetControler(),LOCATION_MZONE,tc:GetControler(),LOCATION_REASON_CONTROL,zone)<=0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_controler(d, tc, 1 - tp) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    let tcp = card::get_controler(d, tc);
    if duel::get_location_count(d, tcp, LOCATION_MZONE as u8, Some(tcp), Some(LOCATION_REASON_CONTROL), Some(zone)).0 <= 0 {
        return;
    }
    // local i=0
    let mut i: u32 = 0;
    // if not c:IsControler(tp) then i=16 end
    if !card::is_controler(d, c, tp) {
        i = 16;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    // local nseq=math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,LOCATION_MZONE,~(zone<<i)),2)-i
    let field = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, LOCATION_MZONE, Some(!(zone << i)), false);
    let nseq = (field as f64).log2() - i as f64;
    // Duel.MoveSequence(tc,nseq)
    duel::move_sequence(d, tc, nseq as u8, None);
}
