//! Earthbound Immortal Cusillu
use super::prelude::*;

pub const ID: u32 = 33537328;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND_IMMORTAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // There can only be 1 "Earthbound Immortal" on the field
    // c:SetUniqueOnField(1,1,aux.FilterBoolFunction(Card.IsSetCard,SET_EARTHBOUND_IMMORTAL),LOCATION_MZONE)
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Function(lua_fn(unique_filter)), Some(LOCATION_MZONE));
    // Destroy it if no face-up Field Spell is on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e1, EFFECT_SELF_DESTROY);
    // e1:SetCondition(s.sdcon)
    effect::set_condition(d, e1, Some(condition(sdcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be targeted for attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(aux.imval2)
    effect::set_value_fn(d, e2, aux::imval2());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can attack directly
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destruction replacement
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `aux.FilterBoolFunction(Card.IsSetCard,SET_EARTHBOUND_IMMORTAL)` as the unique filter
fn unique_filter(d: &mut Duel, a: Args) -> bool {
    card::is_set_card(d, a.c(0), &[SET_EARTHBOUND_IMMORTAL as u16])
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // return not Duel.IsExistingMatchingCard(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsReason(REASON_BATTLE) and c:GetBattlePosition()~=POS_FACEUP_DEFENSE
    //     and Duel.CheckReleaseGroup(tp,Card.IsReleasableByEffect,1,c) end
    if a.chk == Some(0) {
        return card::is_reason(d, c, REASON_BATTLE)
            && card::get_battle_position(d, c) != POS_FACEUP_DEFENSE as u32
            && duel::check_release_group(d, tp, Some(&|d, x| card::is_releasable_by_effect(d, x, None, None)), 1, duel::ReleaseArgs::default(), c.into(), false);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // local g=Duel.SelectReleaseGroup(tp,Card.IsReleasableByEffect,1,1,c)
        let g = duel::select_release_group(d, tp, Some(&|d, x| card::is_releasable_by_effect(d, x, None, None)), 1, 1, duel::ReleaseArgs::default(), c.into(), false).nil();
        // Duel.Release(g,REASON_EFFECT)
        duel::release(d, g, REASON_EFFECT, None);
        // Duel.SetLP(1-tp,Duel.GetLP(1-tp)/2)
        let lp = duel::get_lp(d, 1 - tp);
        duel::set_lp(d, 1 - tp, lua_int(lp as f64 / 2.0) as i32);
        // return true
        true
    } else {
        // return false
        false
    }
}
