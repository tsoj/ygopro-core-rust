//! Evil HERO Wild Cyclone
use super::prelude::*;

pub const ID: u32 = 13293158;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO],
    dark_calling: true,
    listed_names: &[CARD_DARK_FUSION, 21844576, 86188410],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion material
    // Fusion.AddProcMix(c,true,true,21844576,86188410)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(21844576), fusion::Mat::Code(86188410)]);
    // lizard check
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(CARD_CLOCK_LIZARD)
    effect::set_code(d, e0, CARD_CLOCK_LIZARD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCondition(s.lizcon)
    effect::set_condition(d, e0, Some(condition(lizcon)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.EvilHeroLimit)
    effect::set_value_fn(d, e1, aux::evil_hero_limit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent Spell/Traps activation while battling
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(s.aclimit)
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy all face-down Spell/Trap
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.activate)
    effect::set_operation(d, e3, Some(operation(activate)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),EFFECT_SUPREME_CASTLE) end`
fn lizcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_player_affected_by_effect(d, p, EFFECT_SUPREME_CASTLE).is_none()
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFacedown,tp,0,LOCATION_SZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, 0, LOCATION_SZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFacedown,tp,0,LOCATION_SZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, 0, LOCATION_SZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
