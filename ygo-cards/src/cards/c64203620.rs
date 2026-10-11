//! Jormungardr the Nordic Serpent
//! 極星邪竜ヨルムンガンド
use super::prelude::*;

pub const ID: u32 = 64203620;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AESIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTargetRange(POS_FACEUP_DEFENSE,1)
    effect::set_target_range(d, e1, POS_FACEUP_DEFENSE, 1);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| spcon(d, a.e(0), a.card(1)))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // self destory
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e4, EFFECT_SELF_DESTROY);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // damage
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e5, CATEGORY_DAMAGE);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e5, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e5:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e5, EVENT_CHANGE_POS);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(s.damcon)
    effect::set_condition(d, e5, Some(condition(damcon)));
    // e5:SetTarget(s.damtg)
    effect::set_target(d, e5, Some(cost_target(damtg)));
    // e5:SetOperation(s.damop)
    effect::set_operation(d, e5, Some(operation(damop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AESIR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AESIR as u16])
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.descon(e)`
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    !duel::is_existing_matching_card(d, Some(&filter), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPosition(POS_FACEUP_ATTACK) and e:GetHandler():IsPreviousPosition(POS_FACEUP_DEFENSE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_position(d, h, POS_FACEUP_ATTACK) && card::is_previous_position(d, h, POS_FACEUP_DEFENSE)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(3000)
    duel::set_target_param(d, 3000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,3000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 3000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
