//! Michion, the Timelord
use super::prelude::*;

pub const ID: u32 = 7733560;

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
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
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
    // damage val
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    effect::set_code(d, e5, EFFECT_AVOID_BATTLE_DAMAGE);
    card::register_effect(d, c, e5, false);
    // lp
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_range(d, e6, LOCATION_MZONE);
    effect::set_condition(d, e6, Some(condition(lpcon)));
    effect::set_operation(d, e6, Some(operation(lpop)));
    card::register_effect(d, c, e6, false);
    // to deck
    let e7 = effect::create_effect(d, c);
    effect::set_description(d, e7, aux::stringid(ID, 1));
    effect::set_category(d, e7, CATEGORY_TODECK);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e7, EVENT_PHASE | PHASE_STANDBY);
    effect::set_property(d, e7, EFFECT_FLAG_REPEAT, 0);
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_condition(d, e7, Some(condition(tdcon)));
    effect::set_target(d, e7, Some(cost_target(tdtg)));
    effect::set_operation(d, e7, Some(operation(tdop)));
    card::register_effect(d, c, e7, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,0)==0
    //     and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let tp = card::get_controler(d, c);
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.lpcon(e,tp,eg,ep,ev,re,r,rp)`
fn lpcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // Duel.SetLP(1-tp,math.ceil(Duel.GetLP(1-tp)/2))
    let p = 1 - a.tp;
    let lp = duel::get_lp(d, p);
    duel::set_lp(d, p, (lp as f64 / 2.0).ceil() as i32);
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
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from(h)), 1, 0, 0);
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
