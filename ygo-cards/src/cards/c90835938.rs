//! Starry Night, Starry Dragon
use super::prelude::*;

pub const ID: u32 = 90835938;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If Normal or Special Summoned from hand, destroy 1 card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
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
    // Cannot be destroyed by battle with DARKs
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(function(_,c)return c and c:IsAttribute(ATTRIBUTE_DARK)end)
    effect::set_value_fn(d, e3, lua_fn(battle_indes_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed by DARKs' effects
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // e4:SetValue(function(_,re)return re:GetHandler():IsAttribute(ATTRIBUTE_DARK)end)
    effect::set_value_fn(d, e4, lua_fn(effect_indes_value));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Banish attacked monster until EP and chain attack
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e5, CATEGORY_REMOVE);
    // e5:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e5:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e5, EVENT_BATTLE_START);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.bantg)
    effect::set_target(d, e5, Some(cost_target(bantg)));
    // e5:SetOperation(s.banop)
    effect::set_operation(d, e5, Some(operation(banop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_HAND)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_HAND)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// `s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // if chk==0 then return c==Duel.GetAttacker() and tc and tc:IsControler(1-tp) and tc:IsAbleToRemove(tp) end
    if a.chk == Some(0) {
        return duel::get_attacker(d) == Some(c)
            && match tc {
                Some(tc) => card::is_controler(d, tc, 1 - tp) && card::is_able_to_remove(d, tc, Some(tp), None, None),
                None => false,
            };
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc.map(CardSet::from_card), 1, tp, 0);
    true
}
/// `s.banop(e,tp,eg,ep,ev,re,r,rp)`
fn banop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local d=Duel.GetAttacker():GetBattleTarget()
    let att = duel::get_attacker(d).nil();
    let dd = card::get_battle_target(d, att);
    // if d and d:IsRelateToBattle() and d:IsControler(1-tp) and Duel.Remove(d,0,REASON_EFFECT|REASON_TEMPORARY)>0 then
    if let Some(dc) = dd {
        if card::is_relate_to_battle(d, dc)
            && card::is_controler(d, dc, 1 - tp)
            && duel::remove(d, CardSet::from_card(dc), 0, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0
        {
            // d:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            card::register_flag_effect(d, dc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // e1:SetLabelObject(d)
            effect::set_label_object(d, e1, Value::Card(dc));
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetCondition(s.retcon)
            effect::set_condition(d, e1, Some(condition(retcon)));
            // e1:SetOperation(s.retop)
            effect::set_operation(d, e1, Some(operation(retop)));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
    // if c:IsRelateToEffect(e) and c:CanChainAttack() then
    if card::is_relate_to_effect(d, c, e) && card::can_chain_attack(d, c, Some(2), false) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetOperation(function(e) Duel.ChainAttack() end)
        effect::set_operation(d, e2, Some(lua_fn(chainattack_op)));
        // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `function(e) Duel.ChainAttack() end` of `s.banop`
fn chainattack_op(d: &mut Duel, _a: Args) {
    duel::chain_attack(d, None);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetFlagEffect(id)~=0
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    card::get_flag_effect(d, lo, ID) != 0
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, lo, None, None);
}
/// `function(_,c)return c and c:IsAttribute(ATTRIBUTE_DARK)end` of `e3:SetValue`
fn battle_indes_value(d: &mut Duel, a: Args) -> bool {
    // c and c:IsAttribute(ATTRIBUTE_DARK)
    match a.card(1) {
        Some(c) => card::is_attribute(d, c, ATTRIBUTE_DARK),
        None => false,
    }
}
/// `function(_,re)return re:GetHandler():IsAttribute(ATTRIBUTE_DARK)end` of `e4:SetValue`
fn effect_indes_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // re:GetHandler():IsAttribute(ATTRIBUTE_DARK)
    let h = effect::get_handler(d, re).nil();
    card::is_attribute(d, h, ATTRIBUTE_DARK)
}
