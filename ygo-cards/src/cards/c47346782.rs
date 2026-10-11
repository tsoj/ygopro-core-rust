//! Centerfrog
use super::prelude::*;

pub const ID: u32 = 47346782;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
    // e1:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
    effect::set_value_fn(d, e1, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change battle position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Change control
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e4, CATEGORY_CONTROL);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.cttg)
    effect::set_target(d, e4, Some(cost_target(cttg)));
    // e4:SetOperation(s.ctop)
    effect::set_operation(d, e4, Some(operation(ctop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() and c:IsRelateToEffect(e) then
    if card::is_attack_pos(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.ctfilter1(c,tp)`
fn ctfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local seq=c:GetSequence()
    let seq = card::get_sequence(d, c);
    // if seq>4 then return false end
    if seq > 4 {
        return false;
    }
    // return (seq>0 and Duel.CheckLocation(tp,LOCATION_MZONE,seq-1))
    //   or (seq<4 and Duel.CheckLocation(tp,LOCATION_MZONE,seq+1))
    (seq > 0 && duel::check_location(d, tp, LOCATION_MZONE, seq - 1)) || (seq < 4 && duel::check_location(d, tp, LOCATION_MZONE, seq + 1))
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.ctfilter1(chkc,1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && ctfilter1(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ctfilter1,tp,0,LOCATION_MZONE,1,nil,1-tp)
    //   and e:GetHandler():IsControlerCanBeChanged() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&|d, x| ctfilter1(d, x, 1 - tp)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && card::is_controler_can_be_changed(d, h, false, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.ctfilter1,tp,0,LOCATION_MZONE,1,1,nil,1-tp)
    let _g = duel::select_target(d, tp, Some(&|d, x| ctfilter1(d, x, 1 - tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.ctfilter2(c)`
fn ctfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5 and c:IsFaceup() and c:IsCode(id)
    card::get_sequence(d, c) < 5 && card::is_faceup(d, c) && card::is_code(d, c, &[ID])
}
/// `s.ctfilter3(c,seq1,seq2)`
fn ctfilter3(d: &mut Duel, c: CardId, seq1: u32, seq2: u32) -> bool {
    // local seq=c:GetSequence()
    // return seq>seq1 and seq<seq2 and c:IsControlerCanBeChanged()
    let seq = card::get_sequence(d, c);
    seq > seq1 && seq < seq2 && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or c:IsControler(1-tp) or not c:IsPosition(POS_FACEUP_DEFENSE) then return end
    if !card::is_relate_to_effect(d, c, e) || card::is_controler(d, c, 1 - tp) || !card::is_position(d, c, POS_FACEUP_DEFENSE) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) or tc:IsControler(tp) then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, e) || card::is_controler(d, tc, tp) {
        return;
    }
    // local seq=tc:GetSequence()
    let seq = card::get_sequence(d, tc);
    // if seq>4 then return end
    if seq > 4 {
        return;
    }
    // local zone=0
    let mut zone: u32 = 0;
    // if seq>0 then zone=bit.replace(zone,0x1,seq-1) end
    if seq > 0 {
        zone |= 1 << (seq - 1);
    }
    // if seq<4 then zone=bit.replace(zone,0x1,seq+1) end
    if seq < 4 {
        zone |= 1 << (seq + 1);
    }
    // if Duel.GetControl(c,1-tp,0,0,zone)==0 then return end
    if duel::get_control(d, c, 1 - tp, 0, 0, Some(zone), None) == 0 {
        return;
    }
    // local g1=Duel.GetMatchingGroup(s.ctfilter2,tp,0,LOCATION_MZONE,nil)
    let g1 = duel::get_matching_group(d, Some(&ctfilter2), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g1==2 then
    if g1.len() == 2 {
        // local seq1=g1:GetFirst():GetSequence()
        // local seq2=g1:GetNext():GetSequence()
        let cards = group::iter(d, &g1);
        let mut seq1 = card::get_sequence(d, cards[0]);
        let mut seq2 = card::get_sequence(d, cards[1]);
        // if seq2<seq1 then seq1,seq2=seq2,seq1 end
        if seq2 < seq1 {
            std::mem::swap(&mut seq1, &mut seq2);
        }
        // local g2=Duel.GetMatchingGroup(s.ctfilter3,tp,0,LOCATION_MZONE,nil,seq1,seq2)
        let g2 = duel::get_matching_group(d, Some(&|d, x| ctfilter3(d, x, seq1, seq2)), tp, 0, LOCATION_MZONE, NO_EX);
        // if #g2>0 then
        if !g2.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.GetControl(g2,tp)
            duel::get_control(d, g2, tp, 0, 0, None, None);
        }
    }
}
