//! Goldilocks the Battle Landscaper
use super::prelude::*;

pub const ID: u32 = 24521754;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // self destroy on summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // move
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
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
    // atk
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetCondition(aux.NOT(s.descon))
    effect::set_condition(d, e4, Some(condition(|d, a| !descon(d, a))));
    // e4:SetValue(3000)
    effect::set_value(d, e4, 3000);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.descon(e)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSequence()~=2
    let h = effect::get_handler(d, a.e).nil();
    card::get_sequence(d, h) != 2
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
    // local seq=Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,0)
    let seq = duel::select_disable_field(d, a.tp, 1, LOCATION_MZONE, 0, None, false);
    // Duel.Hint(HINT_ZONE,tp,seq)
    duel::hint(d, HINT_ZONE as u8, a.tp, seq as u64);
    // e:SetLabel(math.log(seq,2))
    effect::set_label(d, a.e, &[(seq as f64).log2() as i64]);
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local seq2=e:GetLabel()
    let seq2 = effect::get_label(d, a.e);
    // if not c:IsRelateToEffect(e) or c:IsControler(1-tp) or c:IsImmuneToEffect(e) or not Duel.CheckLocation(tp,LOCATION_MZONE,seq2) then return end
    if !card::is_relate_to_effect(d, c, a.e)
        || card::is_controler(d, c, 1 - a.tp)
        || card::is_immune_to_effect(d, c, a.e)
        || !duel::check_location(d, a.tp, LOCATION_MZONE, seq2 as u32)
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
    // local seq1=c:GetSequence() --register the sequence it comes from
    let seq1 = card::get_sequence(d, c);
    // local dg=c:GetColumnGroup() --and the group of cards there
    let mut dg = card::get_column_group(d, c, 0, 0);
    // Duel.MoveSequence(c,seq2)
    duel::move_sequence(d, c, seq2 as u8, None);
    // if c:GetSequence()==seq2 and seq1~=seq2 then
    if card::get_sequence(d, c) as i64 == seq2 && seq1 as i64 != seq2 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local diff=math.abs(seq1-seq2)
        let diff = (seq1 as i64 - seq2).abs() as u8;
        if seq1 as i64 > seq2 {
            // dg:Merge(c:GetColumnGroup(nil,diff))
            let cg = card::get_column_group(d, c, 0, diff);
            group::add_card(&mut dg, &cg);
        } else {
            // dg:Merge(c:GetColumnGroup(diff,nil))
            let cg = card::get_column_group(d, c, diff, 0);
            group::add_card(&mut dg, &cg);
        }
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
