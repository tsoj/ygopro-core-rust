//! Frightfur Bear
use super::prelude::*;

pub const ID: u32 = 85545073;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_EDGE_IMP, SET_FLUFFAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,30068120,3841833)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(30068120), fusion::Mat::Code(3841833)]);
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e1, Some(condition(eqcon)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,aux.FilterBoolFunction(Card.IsMonster),s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_monster(d, a.c(0))),
        lua_fn(|d, a| {
            equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
            Value::Nil
        }),
        e1,
        None,
        None,
        None,
    );
}
// function s.eqcon(e,tp,eg,ep,ev,re,r,rp)
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and c:IsFaceup() and tc:IsLocation(LOCATION_GRAVE) and tc:IsMonster() and tc:IsReason(REASON_BATTLE)
    card::is_relate_to_battle(d, c)
        && card::is_faceup(d, c)
        && card::is_location(d, tc.nil(), LOCATION_GRAVE)
        && card::is_monster(d, tc.nil())
        && card::is_reason(d, tc.nil(), REASON_BATTLE)
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // local tc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, c).nil();
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
// function s.equipop(c,e,tp,tc)
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value(d, e2, 1000);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        equipop(d, c, a.e, a.tp, tc.nil());
    }
}
