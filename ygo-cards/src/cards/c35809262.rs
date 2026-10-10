//! Elemental HERO Flame Wingman
use super::prelude::*;

pub const ID: u32 = 35809262;
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(21844576), fusion::Mat::Code(58932615)]);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux::fuslimit());
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
/// `s.damcon(e,...)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    card::is_relate_to_battle(d, c) && card::is_location(d, bc.nil(), LOCATION_GRAVE) && card::is_monster(d, bc.nil())
}
/// `s.damtg(e,...,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc);
    // local dam=bc:GetAttack()
    let mut dam = card::get_attack(d, bc);
    // if dam<0 then dam=0 end
    if dam < 0 {
        dam = 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
/// `s.damop(e,...)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
        let p = info[0].int() as u8;
        // local dam=tc:GetAttack()
        let mut dam = card::get_attack(d, tc);
        // if dam<0 then dam=0 end
        if dam < 0 {
            dam = 0;
        }
        // Duel.Damage(p,dam,REASON_EFFECT)
        duel::damage(d, p, dam as i64, REASON_EFFECT, false, None);
    }
}
