//! Nekroz of Metaltron
use super::prelude::*;

pub const ID: u32 = 13408726;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEKROZ], mat_filter: Some(mat_filter), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // c:AddMustBeRitualSummoned()
    card::add_must_be_ritual_summoned(d, c);
    // Banish 1 Spell/Trap on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_REMOVE)
    effect::set_code(d, e1, EVENT_REMOVE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.strmcon)
    effect::set_condition(d, e1, Some(condition(strmcon)));
    // e1:SetTarget(s.strmtg)
    effect::set_target(d, e1, Some(cost_target(strmtg)));
    // e1:SetOperation(s.strmop)
    effect::set_operation(d, e1, Some(operation(strmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish this card until the End Phase and banish 1 face-up monster your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(main_cond)));
    // e2:SetTarget(s.mrmtg)
    effect::set_target(d, e2, Some(cost_target(mrmtg)));
    // e2:SetOperation(s.mrmop)
    effect::set_operation(d, e2, Some(operation(mrmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Any monster destroyed by battle with your "Nekroz" monster is banished
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCode(EFFECT_BATTLE_DESTROY_REDIRECT)
    effect::set_code(d, e3, EFFECT_BATTLE_DESTROY_REDIRECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e3, LOCATION_REMOVED as i64);
    // e3:SetTarget(function(e,c) return c:IsSetCard(SET_NEKROZ) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_NEKROZ as u16]))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.mat_filter(c)`
fn mat_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return not c:IsLevel(9)
    !card::is_level(d, c, &[9])
}
/// `s.strmconfilter(c,tp)`
fn strmconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NEKROZ) and c:IsPreviousControler(tp) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_previous_controler(d, c, tp) && card::is_faceup(d, c)
}
/// `s.strmcon(e,tp,eg,ep,ev,re,r,rp)`
fn strmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.strmconfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| strmconfilter(d, c, a.tp), 1, NO_EX)
}
/// `aux.AND(Card.IsSpellTrap,Card.IsAbleToRemove)`
fn spelltrap_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.strmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn strmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsSpellTrap,Card.IsAbleToRemove),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&spelltrap_removable), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsSpellTrap,Card.IsAbleToRemove),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&spelltrap_removable), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, a.tp, 0);
    true
}
/// `s.strmop(e,tp,...)`
fn strmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `function() return Duel.IsMainPhase() end`
fn main_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `aux.FaceupFilter(Card.IsAbleToRemove)`
fn faceup_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.mrmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mrmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && faceup_removable(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToRemove),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && duel::is_existing_target(d, Some(&faceup_removable), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToRemove),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&faceup_removable), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g+c,2,tp,0)
    let mut gc = g;
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(gc), 2, a.tp, 0);
    true
}
/// `s.mrmop(e,tp,...)`
fn mrmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and aux.RemoveUntil(c,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
    //     and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && aux::remove_until(d, c, None, REASON_EFFECT, PHASE_END, ID, a.e, a.tp, Some(aux::default_field_return()), None, None, None, None, None).is_some()
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc.nil(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
