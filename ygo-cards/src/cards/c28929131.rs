//! Zaphion, the Timelord
use super::prelude::*;

pub const ID: u32 = 28929131;

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e2:SetCondition(s.ntcon)
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
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
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_AVOID_BATTLE_DAMAGE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // to deck
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e6, CATEGORY_TODECK);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCondition(s.tdcon)
    effect::set_condition(d, e6, Some(condition(tdcon)));
    // e6:SetTarget(s.tdtg)
    effect::set_target(d, e6, Some(cost_target(tdtg)));
    // e6:SetOperation(s.tdop)
    effect::set_operation(d, e6, Some(operation(tdop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // draw
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e7, aux::stringid(ID, 2));
    // e7:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e7, CATEGORY_DRAW);
    // e7:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e7:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e7, EFFECT_FLAG_DELAY, 0);
    // e7:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e7, EVENT_TO_GRAVE);
    // e7:SetCountLimit(1)
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    // e7:SetCondition(s.drcon)
    effect::set_condition(d, e7, Some(condition(drcon)));
    // e7:SetTarget(s.drtg)
    effect::set_target(d, e7, Some(cost_target(drtg)));
    // e7:SetOperation(s.drop)
    effect::set_operation(d, e7, Some(operation(drop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // to deck
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e8, aux::stringid(ID, 3));
    // e8:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e8, CATEGORY_TODECK);
    // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e8:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e8, EVENT_PHASE | PHASE_STANDBY);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCondition(s.rtdcon)
    effect::set_condition(d, e8, Some(condition(rtdcon)));
    // e8:SetTarget(s.rtdtg)
    effect::set_target(d, e8, Some(cost_target(rtdtg)));
    // e8:SetOperation(s.rtdop)
    effect::set_operation(d, e8, Some(operation(rtdop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let minc = a.int(2);
    // return minc==0 and c:GetLevel()>4
    //     and Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,0)==0
    //     and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    if minc != 0 {
        return false;
    }
    if card::get_level(d, c) <= 4 {
        return false;
    }
    let tp = card::get_controler(d, c);
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToDeck()
    card::is_spell_trap(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
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
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dn = info[1].int() as u32;
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, dn, REASON_EFFECT);
}
/// `s.rtdcon(e,tp,eg,ep,ev,re,r,rp)`
fn rtdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.rtdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rtdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.rtdop(e,tp,eg,ep,ev,re,r,rp)`
fn rtdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
