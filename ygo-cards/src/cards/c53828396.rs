//! Adhesive Explosive
use super::prelude::*;

pub const ID: u32 = 53828396;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()==e:GetHandler() and e:GetHandler():GetBattlePosition()==POS_FACEDOWN_DEFENSE
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attack_target(d) == Some(h) && card::get_battle_position(d, h) == POS_FACEDOWN_DEFENSE
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // if not tc:IsRelateToBattle() or not c:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, tc) || !card::is_relate_to_battle(d, c) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:IsFacedown() then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 || card::is_facedown(d, tc) {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
        // return
        return;
    }
    // Duel.Equip(tp,c,tc)
    duel::equip(d, tp, c, tc, true, false);
    // local e1=Effect.CreateEffect(tc)
    let e1 = effect::create_effect(d, tc);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(s.eqlimit)
    effect::set_value_fn(d, e1, lua_fn(eqlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy&damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetOwner()==c
    let e = a.e(0);
    effect::get_owner(d, e) == a.card(1)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // ec:CreateEffectRelation(e)
    card::create_effect_relation(d, ec, e);
    // e:SetLabelObject(ec)
    effect::set_label_object(d, e, Value::Card(ec));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,ec,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(ec)), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, e).as_card().nil();
    // if ec:IsRelateToEffect(e) and ec:IsFaceup() then
    if card::is_relate_to_effect(d, ec, e) && card::is_faceup(d, ec) {
        // if Duel.Destroy(ec,REASON_EFFECT)~=0 then
        if duel::destroy(d, ec, REASON_EFFECT, None, None) != 0 {
        } else {
            // Duel.Destroy(c,REASON_EFFECT)
            duel::destroy(d, c, REASON_EFFECT, None, None);
        }
    }
}
