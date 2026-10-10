//! Fusion Duplication
use super::prelude::*;

pub const ID: u32 = 43331750;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cpfilter(c)`
fn cpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FUSION) and (c:IsNormalSpell() or c:IsQuickPlaySpell()) and c:IsAbleToRemove()
    //     and c:CheckActivateEffect(true,true,false)~=nil
    card::is_set_card(d, c, &[SET_FUSION as u16])
        && (card::is_normal_spell(d, c) || card::is_quick_play_spell(d, c))
        && card::is_able_to_remove(d, c, None, None, None)
        && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local te=e:GetLabelObject():GetFirst():CheckActivateEffect(true,true,false)
        let lo = effect::get_label_object(d, a.e);
        let g = lo.as_group().cloned().nil();
        let first = group::get_first(d, &g).nil();
        let te = card::check_activate_effect(d, first, true, true, false).nil().0;
        // local tg=te:GetTarget()
        let tg = effect::get_target(d, te).nil();
        // return tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        let args = [
            Value::Effect(a.e),
            Value::Int(tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(0),
            Value::Card(chkc),
        ];
        return tg.call(d, &args).truthy();
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cpfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cpfilter), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectTarget(tp,s.cpfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil):GetFirst()
    let g = duel::select_target(d, tp, Some(&cpfilter), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // local te=tc:CheckActivateEffect(true,true,false)
    let te = card::check_activate_effect(d, tc, true, true, false).map(|x| x.0);
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET|te:GetProperty())
    let te = te.nil();
    let prop = effect::get_property(d, te).0;
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET | prop, 0);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // if tg then
    if let Some(tg) = tg {
        // tg(e,tp,eg,ep,ev,re,r,rp,1)
        let args = [
            Value::Effect(a.e),
            Value::Int(tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(1),
        ];
        tg.call(d, &args);
    }
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(tc)), 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if not tc:IsRelateToEffect(e) or Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local te=tc:CheckActivateEffect(true,true,false)
    // if not te then return end
    let Some((te, _)) = card::check_activate_effect(d, tc, true, true, false) else {
        return;
    };
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = op {
        let args = [
            Value::Effect(a.e),
            Value::Int(tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
        ];
        op.call(d, &args);
    }
}
