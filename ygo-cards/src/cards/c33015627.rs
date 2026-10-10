//! Sandaion, the Timelord
use super::prelude::*;

pub const ID: u32 = 33015627;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetRange(LOCATION_DECK)
    effect::set_range(d, e1, LOCATION_DECK);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // damage val
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_NO_BATTLE_DAMAGE);
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE);
    // e8:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e8, EFFECT_AVOID_BATTLE_DAMAGE);
    // e8:SetValue(1)
    effect::set_value(d, e8, 1);
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // LP
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e6, CATEGORY_DAMAGE);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCondition(s.damcon)
    effect::set_condition(d, e6, Some(condition(damcon)));
    // e6:SetTarget(s.damtg)
    effect::set_target(d, e6, Some(cost_target(damtg)));
    // e6:SetOperation(s.damop)
    effect::set_operation(d, e6, Some(operation(damop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // to deck
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e7, aux::stringid(ID, 1));
    // e7:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e7, CATEGORY_TODECK);
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e7:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e7, EVENT_PHASE | PHASE_STANDBY);
    // e7:SetProperty(EFFECT_FLAG_REPEAT)
    effect::set_property(d, e7, EFFECT_FLAG_REPEAT, 0);
    // e7:SetCountLimit(1)
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCondition(s.tdcon)
    effect::set_condition(d, e7, Some(condition(tdcon)));
    // e7:SetTarget(s.tdtg)
    effect::set_target(d, e7, Some(cost_target(tdtg)));
    // e7:SetOperation(s.tdop)
    effect::set_operation(d, e7, Some(operation(tdop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,0)==0
    //     and Duel.GetFieldGroupCount(c:GetControler(),0,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) == 0
        && duel::get_field_group_count(d, p, 0, LOCATION_MZONE) > 0
        && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(2000)
    duel::set_target_param(d, 2000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,2000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 2000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:IsAbleToDeck() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_able_to_deck(d, c) {
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
