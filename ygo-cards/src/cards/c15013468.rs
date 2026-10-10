//! Andro Sphinx
use super::prelude::*;

pub const ID: u32 = 15013468;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[53569894], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE | EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    card::register_effect(d, c, e2, false);
    // damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e3, Some(condition(damcon)));
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
    // cannot attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e4, Some(operation(atklimit)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e5, false);
}
fn atklimit(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, h, e1, false);
}
/// `function(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and Duel.CheckLPCost(c:GetControler(),500)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,53569894),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::check_lp_cost(d, tp, 500)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[53569894]))), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // Duel.PayLPCost(tp,500)
    let tp = a.p(1);
    duel::pay_lp_cost(d, tp, 500);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and (bc:GetBattlePosition()&POS_DEFENSE)~=0
    card::is_relate_to_battle(d, c) && (card::get_battle_position(d, bc.nil()) & POS_DEFENSE) != 0
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // local dam=e:GetHandler():GetBattleTarget():GetBaseAttack()/2
    let bc = card::get_battle_target(d, effect::get_handler(d, a.e).nil()).nil();
    let atk = card::get_base_attack(d, bc);
    // Lua's "/" is a float; the core rounds it (half away from zero)
    let dam = lua_int(atk as f64 / 2.0);
    // if dam<0 then dam=0 end
    let dam = dam.max(0);
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
