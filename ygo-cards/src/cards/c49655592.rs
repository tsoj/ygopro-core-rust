//! F.A. Whip Crosser
use super::prelude::*;

pub const ID: u32 = 49655592;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --activate cost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_ACTIVATE_COST)
    effect::set_code(d, e2, EFFECT_ACTIVATE_COST);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCost(s.costchk)
    effect::set_cost(d, e2, Some(lua_fn(|d, a: Args| costchk(d, a.e(0), a.p(2)))));
    // e2:SetTarget(s.costtg)
    effect::set_target(d, e2, Some(lua_fn(|d, a: Args| costtg(d, a.e(0), a.e(1)))));
    // e2:SetOperation(s.costop)
    effect::set_operation(d, e2, Some(operation(costop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --accumulate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(id)
    effect::set_code(d, e3, ID);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --increase level
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e5, CATEGORY_LVCHANGE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.lvcon)
    effect::set_condition(d, e5, Some(condition(lvcon)));
    // e5:SetOperation(s.lvop)
    effect::set_operation(d, e5, Some(operation(lvop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // --restricts discarding
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(0,1)
    effect::set_target_range(d, e6, 0, 1);
    // e6:SetCode(EFFECT_CANNOT_DISCARD_HAND)
    effect::set_code(d, e6, EFFECT_CANNOT_DISCARD_HAND);
    // e6:SetCondition(s.excon)
    effect::set_condition(d, e6, Some(condition(excon)));
    // e6:SetValue(1)
    effect::set_value(d, e6, 1);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetTargetRange(0,LOCATION_HAND)
    effect::set_target_range(d, e7, 0, LOCATION_HAND);
    // e7:SetCode(EFFECT_CANNOT_TO_GRAVE_AS_COST)
    effect::set_code(d, e7, EFFECT_CANNOT_TO_GRAVE_AS_COST);
    // e7:SetCondition(s.excon)
    effect::set_condition(d, e7, Some(condition(excon)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*300
    card::get_level(d, c) as i64 * 300
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsSpellTrapEffect() and re:GetHandler():IsSetCard(SET_FA)
    let re = a.re.nil();
    if !effect::is_spell_trap_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_FA as u16])
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.costchk(e,te_or_c,tp)`
fn costchk(d: &mut Duel, _e: EffectId, tp: u8) -> bool {
    // local ct=#{Duel.GetPlayerEffect(tp,id)}
    let ct = duel::get_player_effect(d, tp, ID).len() as u32;
    // return Duel.CheckLPCost(tp,ct*300)
    duel::check_lp_cost(d, tp, ct * 300)
}
/// `s.costtg(e,te,tp)`
fn costtg(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    // if not te:IsMonsterEffect() then return false end
    if !effect::is_monster_effect(d, te) {
        return false;
    }
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te).nil();
    // local lv=e:GetHandler():GetLevel()
    let h = effect::get_handler(d, e).nil();
    let lv = card::get_level(d, h) as i64;
    // if tc:GetRank()>0 then
    if card::get_rank(d, tc) > 0 {
        // return tc:GetOriginalRank()<lv
        (card::get_original_rank(d, tc) as i64) < lv
    } else if card::has_level(d, tc) {
        // return tc:GetOriginalLevel()<lv
        (card::get_original_level(d, tc) as i64) < lv
    } else {
        // else return false end
        false
    }
}
/// `s.costop(e,tp,eg,ep,ev,re,r,rp)`
fn costop(d: &mut Duel, a: &EffArgs) {
    // Duel.PayLPCost(tp,300)
    duel::pay_lp_cost(d, a.tp, 300);
}
/// `s.excon(e)`
fn excon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLevelAbove(7)
    let h = effect::get_handler(d, a.e).nil();
    card::is_level_above(d, h, 7)
}
