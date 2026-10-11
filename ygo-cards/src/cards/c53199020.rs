//! Fabled Dianaira
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 53199020;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FABLED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be Tribute Summoned by Tributing 1 "Fabled" monster
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(Arc::new(otfilter)), None);
    // The effect of a Normal Spell activated by the opponent becomes "Your opponent discards 1 card"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.changecon)
    effect::set_condition(d, e1, Some(condition(changecon)));
    // e1:SetOperation(s.changeop)
    effect::set_operation(d, e1, Some(operation(changeop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.otfilter(c,tp)`
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FABLED) and (c:IsControler(tp) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_FABLED as u16]) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.changecon(e,tp,eg,ep,ev,re,r,rp)`
fn changecon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:GetHandler():IsNormalSpell() and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    if a.ep != 1 - a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.re.nil()).nil();
    card::is_normal_spell(d, h) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
}
/// `s.changeop(e,tp,eg,ep,ev,re,r,rp)`
fn changeop(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.rep_op)
    duel::change_chain_operation(d, a.ev as u8, operation(rep_op));
}
/// `s.rep_op(e,tp,eg,ep,ev,re,r,rp)`
fn rep_op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
    duel::discard_hand(d, 1 - a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
