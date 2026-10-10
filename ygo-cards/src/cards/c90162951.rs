//! Number 35: Ravenous Tarantula
use super::prelude::*;

pub const ID: u32 = 90162951;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(35), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,10,2)
    xyz::add_procedure(d, c, None, Some(10), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Gain ATK and DEF equal to the difference in LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict 600 damage when opponent Special Summons
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy face-up monsters with ATK less than this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Register flag in order to avoid some loops with certain anime cards
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetCode(id)
    effect::set_code(d, e5, ID);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return math.abs(Duel.GetLP(0)-Duel.GetLP(1))
    (duel::get_lp(d, 0) - duel::get_lp(d, 1)).abs() as i64
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetOverlayCount()>0 and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    if card::get_overlay_count(d, c) == 0 {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| card::is_summon_player(d, x, 1 - a.tp), 1, NO_EX)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Damage(1-tp,600,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 600, REASON_EFFECT, false, None);
}
/// `s.desfilter(c,atk)`
fn desfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsAttackBelow(atk)
    card::is_faceup(d, c) && card::is_attack_below(d, c, atk)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,c:GetAttack()) end
    if a.chk == Some(0) {
        let atk = card::get_attack(d, c);
        return duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,c:GetAttack())
    let atk = card::get_attack(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, atk)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,c:GetAttack())
    let atk = card::get_attack(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, atk)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
