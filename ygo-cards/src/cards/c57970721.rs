//! Drytron Asterism
//! 星彩の竜輝巧
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 57970721;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRYTRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCondition(s.descond)
    effect::set_condition(d, e1, Some(condition(descond)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descond()`
fn descond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetAttack()>=1000 and (c:IsSetCard(SET_DRYTRON) or c:IsRitualMonster())
    card::is_faceup(d, c)
        && card::get_attack(d, c) >= 1000
        && (card::is_set_card(d, c, &[SET_DRYTRON as u16]) || card::is_ritual_monster(d, c))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local g1=Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1.clone().nil());
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g2,1,0,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g2, 1, 0, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g1, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local dc=g:GetFirst()
    let order = group::iter(d, &g);
    let mut dc = order.first().copied();
    // if dc==tc then dc=g:GetNext() end
    if dc == tc {
        dc = order.get(1).copied();
    }
    // if tc and tc:UpdateAttack(-1000,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,c)==-1000 and dc and dc:IsControler(1-tp) and not dc:IsFacedown() then
    if let Some(tc) = tc {
        if card::update_attack(d, tc, -1000, Some(RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN), Some(c), None) == -1000 {
            if let Some(dc) = dc {
                if card::is_controler(d, dc, 1 - a.tp) && !card::is_facedown(d, dc) {
                    // Duel.Destroy(dc,REASON_EFFECT)
                    duel::destroy(d, dc, REASON_EFFECT, None, None);
                }
            }
        }
    }
}
