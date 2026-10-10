//! Topologic Bomber Dragon
use super::prelude::*;

pub const ID: u32 = 5821478;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,ec)`
fn cfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // if c:IsLocation(LOCATION_MZONE) then
    if card::is_location(d, c, LOCATION_MZONE) {
        // return ec:GetLinkedGroup():IsContains(c)
        let g = card::get_linked_group(d, ec);
        group::is_contains(&g, c)
    } else {
        // return (ec:GetLinkedZone(c:GetPreviousControler())&(1<<c:GetPreviousSequence()))~=0
        let pc = card::get_previous_controler(d, c);
        let seq = card::get_previous_sequence(d, c);
        (card::get_linked_zone(d, ec, Some(pc)) & (1 << seq)) != 0
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // if eg:IsContains(e:GetHandler()) then return false end
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    if group::is_contains(&eg, h) {
        return false;
    }
    // for tc in aux.Next(Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_LINK)) do
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    for tc in group::iter(d, &g) {
        // if eg:IsExists(s.cfilter,1,nil,tc) then return true end
        if group::is_exists(d, &eg, &|d, c| cfilter(d, c, tc), 1, NO_EX) {
            return true;
        }
    }
    // return false
    false
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.ftarget)
    effect::set_target(d, e1, Some(effect_card_fn(ftarget)));
    // e1:SetLabel(e:GetHandler():GetFieldID())
    let fid = card::get_field_id(d, h) as i64;
    effect::set_label(d, e1, &[fid]);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,2),nil)
    let h = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `s.ftarget(e,c)`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler() and Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e);
    duel::get_attacker(d) == h && duel::get_attack_target(d).is_some()
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,Duel.GetAttackTarget():GetBaseAttack())
    let bc = duel::get_attack_target(d).nil();
    let atk = card::get_base_attack(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // if bc and bc:IsRelateToBattle() and bc:IsFaceup() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) {
            // Duel.Damage(1-tp,bc:GetBaseAttack(),REASON_EFFECT)
            let atk = card::get_base_attack(d, bc);
            duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
