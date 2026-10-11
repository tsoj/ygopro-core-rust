//! Hypnosister
use super::prelude::*;

pub const ID: u32 = 22200403;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk/def
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // e1:SetCondition(s.effcon)
    effect::set_condition(d, e1, Some(condition(|d, a| effcon(d, a.e))));
    // e1:SetLabel(1)
    effect::set_label(d, e1, &[1]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(|d, a| effcon(d, a.e))));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // e3:SetLabel(2)
    effect::set_label(d, e3, &[2]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // must attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e4, EFFECT_MUST_ATTACK);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetCondition(s.effcon)
    effect::set_condition(d, e4, Some(condition(|d, a| effcon(d, a.e))));
    // e4:SetLabel(3)
    effect::set_label(d, e4, &[3]);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e5, EFFECT_MUST_ATTACK_MONSTER);
    // e5:SetValue(s.atklimit)
    effect::set_value_fn(d, e5, effect_card_fn(atklimit));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // draw
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e6, CATEGORY_DRAW);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e6, EVENT_BATTLE_DESTROYING);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetCondition(s.drcon1)
    effect::set_condition(d, e6, Some(condition(drcon1)));
    // e6:SetTarget(s.drtg)
    effect::set_target(d, e6, Some(cost_target(drtg)));
    // e6:SetOperation(s.drop)
    effect::set_operation(d, e6, Some(operation(drop_op)));
    // e6:SetLabel(4)
    effect::set_label(d, e6, &[4]);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e7, aux::stringid(ID, 1));
    // e7:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e7, CATEGORY_DRAW);
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e7:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e7, EVENT_DESTROYED);
    // e7:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e7, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCondition(s.drcon2)
    effect::set_condition(d, e7, Some(condition(drcon2)));
    // e7:SetTarget(s.drtg)
    effect::set_target(d, e7, Some(cost_target(drtg)));
    // e7:SetOperation(s.drop)
    effect::set_operation(d, e7, Some(operation(drop_op)));
    // e7:SetLabel(4)
    effect::set_label(d, e7, &[4]);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
// function s.effcon(e)
fn effcon(d: &mut Duel, e: EffectId) -> bool {
    // return Duel.GetFieldGroupCount(0,LOCATION_PZONE,LOCATION_PZONE)>=e:GetLabel()
    duel::get_field_group_count(d, 0, LOCATION_PZONE, LOCATION_PZONE) as i64 >= effect::get_label(d, e)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // if chk==0 then return tc and tc:IsFaceup() and tc:IsPendulumSummoned() end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::is_faceup(d, tc) && card::is_pendulum_summoned(d, tc),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h).nil();
    // if tc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, tc) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
// function s.atklimit(e,c)
fn atklimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler()
    c == effect::get_handler(d, e).nil()
}
// function s.drcon1(e,tp,eg,ep,ev,re,r,rp)
fn drcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return s.effcon(e) and e:GetHandler():IsRelateToBattle()
    if !effcon(d, a.e) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::is_relate_to_battle(d, h)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
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
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
// function s.drcon2(e,tp,eg,ep,ev,re,r,rp)
fn drcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return s.effcon(e) and (r&REASON_EFFECT)~=0 and re:GetHandler()==e:GetHandler()
    if !effcon(d, a.e) {
        return false;
    }
    if (a.r & REASON_EFFECT) == 0 {
        return false;
    }
    let re_h = effect::get_handler(d, a.re.nil()).nil();
    re_h == effect::get_handler(d, a.e).nil()
}
