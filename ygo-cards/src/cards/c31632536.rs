//! Laval Dual Slasher
use super::prelude::*;

pub const ID: u32 = 31632536;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LAVAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_FIRE),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_FIRE, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // chain attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_operation(d, e1, Some(operation(caop1)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_operation(d, e2, Some(operation(caop2)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
    // pierce
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_PIERCE);
    effect::set_condition(d, e3, Some(condition(pcon)));
    card::register_effect(d, c, e3, false);
}
/// `s.caop1(e,...)`
fn caop1(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if e:GetHandler()==a and d and d:IsDefensePos() then e:SetLabel(1)
    let h = effect::get_handler(d, a.e).nil();
    let def_pos = match dt {
        Some(x) => card::is_defense_pos(d, x),
        None => false,
    };
    if att == Some(h) && def_pos {
        effect::set_label(d, a.e, &[1]);
    } else {
        // else e:SetLabel(0) end
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.caop2(e,...)`
fn caop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabelObject():GetLabel()==1 and c:IsRelateToBattle() and c:CanChainAttack()
    //     and Duel.GetMatchingGroupCount(Card.IsSetCard,tp,LOCATION_GRAVE,0,nil,SET_LAVAL)>=2 then
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) == 1
        && card::is_relate_to_battle(d, c)
        && card::can_chain_attack(d, c, Some(2), false)
        && duel::get_matching_group_count(d, Some(&|d, x| card::is_set_card(d, x, &[SET_LAVAL as u16])), a.tp, LOCATION_GRAVE, 0, NO_EX) >= 2
    {
        // Duel.ChainAttack()
        duel::chain_attack(d, None);
    }
}
/// `s.pcon(e)`
fn pcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroupCount(Card.IsSetCard,e:GetHandler():GetControler(),LOCATION_GRAVE,0,nil,SET_LAVAL)>=3
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_set_card(d, x, &[SET_LAVAL as u16])), p, LOCATION_GRAVE, 0, NO_EX) >= 3
}
