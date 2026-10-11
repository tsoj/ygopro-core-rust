//! Battlewasp - Hama the Conquering Bow
use super::prelude::*;

pub const ID: u32 = 80949182;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLEWASP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Synchro summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // --If synchro summoned using a synchro monster, can make a second attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.tncon)
    effect::set_condition(d, e1, Some(condition(tncon)));
    // e1:SetOperation(s.tnop)
    effect::set_operation(d, e1, Some(operation(tnop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Make all of opponent's monsters lose 1000 ATK/DEF
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.atcon)
    effect::set_condition(d, e3, Some(condition(atcon)));
    // e3:SetTarget(s.attg)
    effect::set_target(d, e3, Some(cost_target(attg)));
    // e3:SetOperation(s.atop)
    effect::set_operation(d, e3, Some(operation(atop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Inflict damage, equal to number of "Battlewasp" monsters in GY x 300
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_BATTLE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.damcon)
    effect::set_condition(d, e4, Some(condition(damcon)));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=true
        set_s_var(d, ID, 0, Value::Bool(true));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DAMAGE)
        effect::set_code(d, ge1, EVENT_BATTLE_DAMAGE);
        // ge1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
        effect::set_property(d, ge1, EFFECT_FLAG_DAMAGE_STEP, 0);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=Effect.CreateEffect(c)
        let ge2 = effect::create_effect(d, c);
        // ge2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge2:SetCode(EVENT_ADJUST)
        effect::set_code(d, ge2, EVENT_ADJUST);
        // ge2:SetCountLimit(1)
        effect::set_count_limit(d, ge2, 1, 0, 0, 0);
        // ge2:SetOperation(s.clear)
        effect::set_operation(d, ge2, Some(operation(clear)));
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,1,nil,TYPE_SYNCHRO) then
    if group::is_exists(d, &g, &|d, cc| card::is_type(d, cc, TYPE_SYNCHRO), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        let le = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, le, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        let le = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, le, &[0]);
    }
    // (the Lua function returns nothing: a value of 0)
    0
}
// function s.tncon(e,tp,eg,ep,ev,re,r,rp)
fn tncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h) && effect::get_label(d, a.e) == 1
}
// function s.tnop(e,tp,eg,ep,ev,re,r,rp)
fn tnop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --Can make a second attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3201)
    effect::set_description(d, e1, 3201);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atcon(e,tp,eg,ep,ev,re,r,rp)
fn atcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp
    a.ep == 1 - a.tp
}
// function s.attg(e,tp,eg,ep,ev,re,r,rp,chk)
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.NOT(aux.FaceupFilter(Card.IsStatus,STATUS_BATTLE_DESTROYED)),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| !(card::is_faceup(d, c) && card::is_status(d, c, STATUS_BATTLE_DESTROYED))),
            a.tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    true
}
// function s.atop(e,tp,eg,ep,ev,re,r,rp)
fn atop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local sc=g:GetFirst()
    let _sc = group::get_first(d, &g);
    // for sc in aux.Next(g) do
    for sc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, handler);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(-1000)
        effect::set_value(d, e1, -1000);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // sc:RegisterEffect(e2)
        card::register_effect(d, sc, e2, false);
    }
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp,chk)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and s[0]
    duel::is_turn_player(d, a.tp) && s_var(d, ID, 0).truthy()
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BATTLEWASP) and c:IsMonster()
    card::is_set_card(d, c, &[SET_BATTLEWASP as u16]) && card::is_monster(d, c)
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local dam=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_GRAVE,0,nil)*300
    let dam = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 300;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local d=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_GRAVE,0,nil)*300
    let dam = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 300;
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if ep~=e:GetHandler():GetControler() then
    let handler = effect::get_handler(d, a.e).nil();
    if a.ep != card::get_controler(d, handler) {
        // s[0]=false
        set_s_var(d, ID, 0, Value::Bool(false));
    }
}
// function s.clear(e,tp,eg,ep,ev,re,r,rp)
fn clear(d: &mut Duel, _a: &EffArgs) {
    // s[0]=true
    set_s_var(d, ID, 0, Value::Bool(true));
}
