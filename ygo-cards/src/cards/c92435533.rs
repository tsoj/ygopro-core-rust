//! Lazion, the Timelord
use super::prelude::*;

pub const ID: u32 = 92435533;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_range(d, e1, LOCATION_DECK);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
    card::register_effect(d, c, e2, false);
    // indes
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    effect::set_code(d, e5, EFFECT_AVOID_BATTLE_DAMAGE);
    card::register_effect(d, c, e5, false);
    // to deck
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 1));
    effect::set_category(d, e6, CATEGORY_TODECK);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_range(d, e6, LOCATION_MZONE);
    effect::set_condition(d, e6, Some(condition(tdcon)));
    effect::set_target(d, e6, Some(cost_target(tdtg)));
    effect::set_operation(d, e6, Some(operation(tdop)));
    card::register_effect(d, c, e6, false);
    // damage
    let e7 = effect::create_effect(d, c);
    effect::set_description(d, e7, aux::stringid(ID, 2));
    effect::set_category(d, e7, CATEGORY_DAMAGE);
    effect::set_property(d, e7, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e7, EVENT_DRAW);
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_condition(d, e7, Some(condition(damcon)));
    effect::set_target(d, e7, Some(cost_target(damtg)));
    effect::set_operation(d, e7, Some(operation(damop)));
    card::register_effect(d, c, e7, false);
    // to deck
    let e8 = effect::create_effect(d, c);
    effect::set_description(d, e8, aux::stringid(ID, 3));
    effect::set_category(d, e8, CATEGORY_TODECK);
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e8, EVENT_PHASE | PHASE_STANDBY);
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    effect::set_range(d, e8, LOCATION_MZONE);
    effect::set_condition(d, e8, Some(condition(rtdcon)));
    effect::set_target(d, e8, Some(cost_target(rtdtg)));
    effect::set_operation(d, e8, Some(operation(rtdop)));
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
    let tp = card::get_controler(d, c);
    minc == 0
        && card::get_level(d, c) > 4
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n as u8, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_GRAVE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dam) = (info[0].int() as u8, info[1].int());
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
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
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from(h)), 1, 0, 0);
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
