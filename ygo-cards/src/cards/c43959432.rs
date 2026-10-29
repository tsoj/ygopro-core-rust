//! Metamorphortress
use super::prelude::*;

pub const ID: u32 = 43959432;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(4)
    card::is_faceup(d, c) && card::is_level_above(d, c, 4)
}
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(1000),
        defense: Some(1000),
        level: Some(4),
        race: Some(RACE_ROCK),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and (Duel.IsDuelType(DUEL_TRAP_MONSTERS_NOT_USE_ZONE) or Duel.GetLocationCount(tp,LOCATION_SZONE)>0)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1000,1000,4,RACE_ROCK,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && (duel::is_duel_type(d, DUEL_TRAP_MONSTERS_NOT_USE_ZONE as u64)
                || duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, monster_data(), None, None, 0);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1000,1000,4,RACE_ROCK,ATTRIBUTE_EARTH) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0)
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // --position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.poscon)
    effect::set_condition(d, e1, Some(condition(poscon)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // --atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2,true)
    card::register_effect(d, c, e2, true);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3,true)
    card::register_effect(d, c, e3, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e,nil,RESET_EVENT|RESETS_STANDARD)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_lua), a.e, None, Some(RESET_EVENT | RESETS_STANDARD), None);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // s.equipop(c,e,tp,tc)
            equipop(d, c, a.e, tp, tc);
        }
    }
}
fn eqval(d: &mut Duel, a: Args) -> bool {
    // function s.eqval(ec,c,tp) return ec:IsControler(tp) and ec:IsLevelAbove(4) end
    let ec = a.c(0);
    card::is_controler(d, ec, a.p(2)) && card::is_level_above(d, ec, 4)
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // c:EquipByEffectAndLimitRegister(e,tp,tc,id)
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None);
}
/// `s.equipop` as called by aux.AddEREquipLimit: `function(c,e,tp,tc)`
fn equipop_lua(d: &mut Duel, a: Args) {
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler()==Duel.GetAttacker() and e:GetHandler():IsRelateToBattle()
    let h = effect::get_handler(d, a.e).nil();
    Some(h) == duel::get_attacker(d) && card::is_relate_to_battle(d, h)
}
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() then
    if card::is_attack_pos(d, c) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, CardSet::from_card(c), POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0
    card::get_flag_effect(d, c, ID) != 0
}
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetEquipGroup():Filter(s.eqfilter,nil)
    let h = effect::get_handler(d, e).nil();
    let eqg = card::get_equip_group(d, h);
    let g = group::filter(d, &eqg, &eqfilter, NO_EX);
    // local atk=0
    let mut atk: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local tatk=tc:GetAttack()
        let tatk = card::get_attack(d, tc) as i64;
        // if tatk>0 then atk=atk+tatk end
        if tatk > 0 {
            atk += tatk;
        }
    }
    // return atk
    atk
}
