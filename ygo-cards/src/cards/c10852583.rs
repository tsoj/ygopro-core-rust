//! Yajiro Invader
//! Some parts remade by Edo9300
use super::prelude::*;

pub const ID: u32 = 10852583;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy itself
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
    // Move itself to another monster zone
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
    // e3:SetCondition(aux.seqmovcon)
    effect::set_condition(d, e3, Some(aux::seqmovcon()));
    // e3:SetOperation(aux.seqmovop)
    effect::set_operation(d, e3, Some(aux::seqmovop()));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy cards after moved
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.mvcon)
    effect::set_condition(d, e4, Some(condition(mvcon)));
    // e4:SetTarget(s.mvtg)
    effect::set_target(d, e4, Some(cost_target(mvtg)));
    // e4:SetOperation(s.mvop)
    effect::set_operation(d, e4, Some(operation(mvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
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
/// `s.mvcon(e,tp,eg,ep,ev,re,r,rp)`
fn mvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return #eg==1 and eg:GetFirst():IsControler(1-tp)
    let eg = a.eg.clone().nil();
    if eg.len() != 1 {
        return false;
    }
    let tc = group::get_first(d, &eg).nil();
    card::is_controler(d, tc, 1 - a.tp)
}
/// `s.mvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:IsLocation(LOCATION_MZONE) then tc:CreateEffectRelation(e) end
    if card::is_location(d, tc, LOCATION_MZONE) {
        card::create_effect_relation(d, tc, a.e);
    }
    true
}
/// `s.mvop(e,tp,eg,ep,ev,re,r,rp)`
fn mvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // if not c:IsRelateToEffect(e) or c:IsControler(1-tp)
    //     or not tc:IsRelateToEffect(e) or tc:IsControler(tp) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - a.tp) {
        return;
    }
    let tc = tc.nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_controler(d, tc, a.tp) {
        return;
    }
    // local seq1=c:GetSequence()
    let seq1 = card::get_sequence(d, c) as i64;
    // local seq2=tc:GetSequence()
    let mut seq2 = card::get_sequence(d, tc) as i64;
    // if seq1>4 then return end
    if seq1 > 4 {
        return;
    }
    // if seq2<5 then seq2=4-seq2
    if seq2 < 5 {
        seq2 = 4 - seq2;
    }
    // elseif seq2==5 then seq2=3
    else if seq2 == 5 {
        seq2 = 3;
    }
    // elseif seq2==6 then seq2=1 end
    else if seq2 == 6 {
        seq2 = 1;
    }
    // local nseq=seq1+(seq2>seq1 and 1 or -1)
    let nseq = seq1 + if seq2 > seq1 { 1 } else { -1 };
    // if seq1~=seq2 and (Duel.CheckLocation(tp,LOCATION_MZONE,nseq)) then
    if seq1 != seq2 && duel::check_location(d, a.tp, LOCATION_MZONE, nseq as u32) {
        // Duel.MoveSequence(c,nseq)
        duel::move_sequence(d, c, nseq as u8, None);
        // local cg=c:GetColumnGroup()
        let cg = card::get_column_group(d, c, 0, 0);
        // if #cg>0 then
        if !cg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(cg,REASON_EFFECT)
            duel::destroy(d, cg, REASON_EFFECT, None, None);
        }
    }
}
