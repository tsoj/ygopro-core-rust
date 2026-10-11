//! Void Seer
use super::prelude::*;

pub const ID: u32 = 8437145;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted "Infernoid" monster becomes unaffected by opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Substitute destruction for 1 of your "Infernoid" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INFERNOID)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INFERNOID as u16])
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, tp) {
            // Unaffected by opponent's card effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3110)
            effect::set_description(d, e1, 3110);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // e1:SetValue(s.efilter)
            effect::set_value_fn(d, e1, lua_fn(efilter));
            // e1:SetOwnerPlayer(tp)
            effect::set_owner_player(d, e1, tp);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetOwnerPlayer()~=re:GetOwnerPlayer()
    effect::get_owner_player(d, a.e(0)) != effect::get_owner_player(d, a.e(1))
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsSetCard(SET_INFERNOID) and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_set_card(d, c, &[SET_INFERNOID as u16])
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, h, None, None, None) && group::is_exists(d, &a.eg(), &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // local g=eg:Filter(s.repfilter,nil,tp)
        let g = group::filter(d, &a.eg(), &|d, c| repfilter(d, c, tp), NO_EX);
        // if #g==1 then e:SetLabelObject(g:GetFirst())
        if g.len() == 1 {
            let first = group::get_first(d, &g).nil();
            effect::set_label_object(d, e, Value::Card(first));
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
            // local cg=g:Select(tp,1,1,nil)
            let cg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // e:SetLabelObject(cg:GetFirst())
            let first = group::get_first(d, &cg).nil();
            effect::set_label_object(d, e, Value::Card(first));
        }
        // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
        duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    Some(c) == effect::get_label_object(d, e).as_card()
}
