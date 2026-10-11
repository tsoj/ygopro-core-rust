//! Graydle Combat
use super::prelude::*;

pub const ID: u32 = 84442536;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAYDLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))) then return false end
    let re = a.re.nil();
    if !(effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0)
        && (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)))
    {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // if not g or #g~=1 then return false end
    let Some(g) = info[0].as_group().cloned() else {
        return false;
    };
    if g.len() != 1 {
        return false;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsControler(tp) and tc:IsLocation(LOCATION_MZONE) and tc:IsFaceup() and tc:IsSetCard(SET_GRAYDLE)
    card::is_controler(d, tc, a.tp)
        && card::is_location(d, tc, LOCATION_MZONE)
        && card::is_faceup(d, tc)
        && card::is_set_card(d, tc, &[SET_GRAYDLE as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // local b1=tc:IsLocation(LOCATION_MZONE)
    let b1 = card::is_location(d, tc, LOCATION_MZONE);
    // local b2=Duel.IsChainNegatable(ev)
    let b2 = duel::is_chain_negatable(d, a.ev as u8);
    // if chk==0 then return tc and (b1 or b2) end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    } else {
        // e:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_NEGATE + CATEGORY_DESTROY);
        // local rc=re:GetHandler()
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
        if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sel=e:GetLabel()
    let sel = effect::get_label(d, a.e);
    // if sel==1 then
    if sel == 1 {
        // Duel.ChangeChainOperation(ev,s.repop)
        duel::change_chain_operation(d, a.ev as u8, lua_fn(repop));
    } else {
        // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
        if duel::negate_activation(d, a.ev as u8) {
            let re = a.re.nil();
            let rc = effect::get_handler(d, re).nil();
            if card::is_relate_to_effect(d, rc, re) {
                // Duel.Destroy(eg,REASON_EFFECT)
                duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // e:GetHandler():CancelToGrave(false)
        let h = effect::get_handler(d, e).nil();
        card::cancel_to_grave(d, h, false);
    }
}
