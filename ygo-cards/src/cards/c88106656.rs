//! Libromancer Fireburst
use super::prelude::*;

pub const ID: u32 = 88106656;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIBROMANCER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Check materials on Ritual Summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.matcheck)
    effect::set_value_fn(d, e0, lua_fn(matcheck));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.matcon)
    effect::set_condition(d, e1, Some(lua_fn(matcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Double battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetCondition(s.matcon)
    effect::set_condition(d, e2, Some(lua_fn(matcon)));
    // e2:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e2, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can make 2 attacks on monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK_MONSTER);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Banish 1 "Libromancer" Ritual monster to gain 200 ATK
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCost(s.atkcost)
    effect::set_cost(d, e4, Some(cost_target(atkcost)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    // if c:GetMaterial():IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) then
    let mat = card::get_material(d, c);
    if group::is_exists(d, &mat, &|d, cc| card::is_location(d, cc, LOCATION_MZONE), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
        card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 0));
    }
    Value::Nil
}
/// `s.matcon(e)`
fn matcon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:IsRitualSummoned() and c:GetFlagEffect(id)>0
    card::is_ritual_summoned(d, c) && card::get_flag_effect(d, c, ID) > 0
}
/// `s.atkcfilter(c)`
fn atkcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualMonster() and c:IsSetCard(SET_LIBROMANCER) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_ritual_monster(d, c)
        && card::is_set_card(d, c, &[SET_LIBROMANCER as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkcfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkcfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_matching_card(d, tp, Some(&atkcfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(200)
        effect::set_value(d, e1, 200);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
