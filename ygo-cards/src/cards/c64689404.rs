//! Sword Breaker
use super::prelude::*;

pub const ID: u32 = 64689404;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,6,2)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.ractg)
    effect::set_target(d, e1, Some(cost_target(ractg)));
    // e1:SetOperation(s.racop)
    effect::set_operation(d, e1, Some(operation(racop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
}
/// `s.ractg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ractg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local lo = e:GetLabelObject()
    let lo = effect::get_label_object(d, e).as_effect().nil();
    // if chk==0 then return e:GetLabelObject():GetLabel()~=RACE_ALL end
    if a.chk == Some(0) {
        return effect::get_label(d, lo) != RACE_ALL as i64;
    }
    // if e:GetHandler():GetFlagEffect(id)==0 then
    let h = effect::get_handler(d, e).nil();
    if card::get_flag_effect(d, h, ID) == 0 {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
    }
    // local prc=e:GetLabelObject():GetLabel()
    let prc = effect::get_label(d, lo);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RACE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RACE as u64);
    // local rc=Duel.AnnounceRace(tp,1,RACE_ALL-prc)
    let rc = duel::announce_race(d, tp, 1, (RACE_ALL as i64 - prc) as u64);
    // e:SetLabel(rc)
    effect::set_label(d, e, &[rc as i64]);
    true
}
/// `s.racop(e,tp,eg,ep,ev,re,r,rp)`
fn racop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let h = effect::get_handler(d, e).nil();
    // if e:GetHandler():IsRelateToEffect(e) and e:GetHandler():IsFaceup() then
    if card::is_relate_to_effect(d, h, e) && card::is_faceup(d, h) {
        // local prc=e:GetLabelObject():GetLabel()
        let lo = effect::get_label_object(d, e).as_effect().nil();
        let prc = effect::get_label(d, lo);
        // local rc=(e:GetLabel()|prc)
        let rc = effect::get_label(d, e) | prc;
        // e:GetLabelObject():SetLabel(rc)
        effect::set_label(d, lo, &[rc]);
        // e:GetHandler():SetHint(CHINT_RACE,rc)
        card::set_hint(d, h, CHINT_RACE as u8, rc as u64);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:GetFlagEffect(id)~=0 and bc and bc:IsFaceup() and bc:IsRace(e:GetLabel())
    if card::get_flag_effect(d, c, ID) == 0 {
        return false;
    }
    match bc {
        Some(bc) => card::is_faceup(d, bc) && card::is_race(d, bc, effect::get_label(d, a.e) as u64),
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler():GetBattleTarget(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bt, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = card::get_battle_target(d, h) {
        if card::is_relate_to_battle(d, bc) {
            // Duel.Destroy(bc,REASON_EFFECT)
            duel::destroy(d, bc, REASON_EFFECT, None, None);
        }
    }
}
