//! Evil HERO Malicious Bane
use super::prelude::*;

pub const ID: u32 = 86165817;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_HERO, SET_EVIL_HERO],
    dark_calling: true,
    listed_names: &[CARD_DARK_FUSION],
    listed_series: &[SET_HERO, SET_EVIL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_EVIL_HERO),aux.FilterBoolFunctionEx(Card.IsLevelAbove,5))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_EVIL_HERO as u16], Some(sc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, _sc, _st, _tp| card::is_level_above(d, x, 5))),
        ],
    );
    // Clock Lizard check
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(CARD_CLOCK_LIZARD)
    effect::set_code(d, e0, CARD_CLOCK_LIZARD);
    // e0:SetCondition(function(e) return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),EFFECT_SUPREME_CASTLE) end)
    effect::set_condition(d, e0, Some(condition(lizcon)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Must be Special Summoned with "Dark Fusion"
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
    // Cannot be destroyed by battle or card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy all monsters your opponent controls with ATK less than or equal to this card's
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),EFFECT_SUPREME_CASTLE) end`
fn lizcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_player_affected_by_effect(d, p, EFFECT_SUPREME_CASTLE).is_none()
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,e:GetHandler():GetAttack()),tp,0,LOCATION_MZONE,nil)
    let h = effect::get_handler(d, a.e).nil();
    let atk = card::get_attack(d, h);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_attack_below(d, x, atk)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot declare an attack for the rest of this turn, except with "HERO" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_HERO) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, x| !card::is_set_card(d, x, &[SET_HERO as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,c:GetAttack()),tp,0,LOCATION_MZONE,nil)
    let atk = card::get_attack(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_attack_below(d, x, atk)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // Gains 200 ATK for each destroyed monster
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(#(Duel.GetOperatedGroup())*200)
        let n = duel::get_operated_group(d).len() as i64;
        effect::set_value(d, e1, n * 200);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
