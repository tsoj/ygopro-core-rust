//! Piercing the Darkness
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 21862633;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop)));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // Increase ATK
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_condition(d, e4, Some(condition(atkcon)));
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE, 0);
        // ge1:SetCode(EFFECT_MATERIAL_CHECK)
        effect::set_code(d, ge1, EFFECT_MATERIAL_CHECK);
        // ge1:SetValue(s.valcheck)
        effect::set_value_fn(d, ge1, lua_fn(valcheck));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, a: Args) {
    let _e = a.e(0);
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,1,nil,TYPE_NORMAL) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_NORMAL), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD),0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 0, 1, 0, 0);
    }
}
// function s.drcfilter(c,tp)
fn drcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and not c:IsType(TYPE_TOKEN) and c:IsSummonPlayer(tp)
    card::is_type(d, c, TYPE_NORMAL) && !card::is_type(d, c, TYPE_TOKEN) && card::is_summon_player(d, c, tp)
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.drcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| drcfilter(d, c, a.tp), 1, NO_EX)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsPlayerCanDraw(tp,1)
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // if c:IsType(TYPE_NORMAL) then return c:IsLevelAbove(5)
    if card::is_type(d, c, TYPE_NORMAL) {
        card::is_level_above(d, c, 5)
    } else {
        // local summon_types={SUMMON_TYPE_RITUAL,SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ}
        // return c:GetFlagEffect(id)>0 and c:IsSummonType(table.unpack(summon_types))
        let summon_types = [SUMMON_TYPE_RITUAL, SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ];
        card::get_flag_effect(d, c, ID) > 0 && card::is_summon_type(d, c, &summon_types)
    }
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let mut x = Some(atk);
    let mut dd = card::get_battle_target(d, atk);
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, atk, 1 - a.tp) {
        std::mem::swap(&mut x, &mut dd);
    }
    // return a and a:IsFaceup() and a:IsRelateToBattle() and s.atkfilter(a)
    //   and d and d:IsFaceup() and d:IsRelateToBattle()
    //   and d:IsAttackAbove(0) and a:GetControler()~=d:GetControler()
    let (Some(x), Some(dd)) = (x, dd) else { return false };
    card::is_faceup(d, x)
        && card::is_relate_to_battle(d, x)
        && atkfilter(d, x)
        && card::is_faceup(d, dd)
        && card::is_relate_to_battle(d, dd)
        && card::is_attack_above(d, dd, 0)
        && card::get_controler(d, x) != card::get_controler(d, dd)
}
// function s.atkop(e,tp,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let mut x = Some(atk);
    let mut dd = card::get_battle_target(d, atk);
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, atk, 1 - a.tp) {
        std::mem::swap(&mut x, &mut dd);
    }
    // if a and a:IsFaceup() and a:IsRelateToBattle() and d and d:IsFaceup() and d:IsRelateToBattle() then
    if let (Some(x), Some(dd)) = (x, dd) {
        if card::is_faceup(d, x) && card::is_relate_to_battle(d, x) && card::is_faceup(d, dd) && card::is_relate_to_battle(d, dd) {
            // Increase ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(d:GetAttack())
            let datk = card::get_attack(d, dd);
            effect::set_value(d, e1, datk as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // a:RegisterEffect(e1)
            card::register_effect(d, x, e1, false);
        }
    }
}
