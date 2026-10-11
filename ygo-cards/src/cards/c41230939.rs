//! Cyberdark Horn
//! サイバー・ダーク・ホーン
use super::prelude::*;

pub const ID: u32 = 41230939;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(Cyberdark.EquipTarget(s.eqfilter,true,true))
    effect::set_target(d, e1, Some(aux::cyberdark::equip_target(Some(eqfilter_arc()), true, true)));
    // e1:SetOperation(Cyberdark.EquipOperation(s.eqfilter,s.equipop,true))
    effect::set_operation(d, e1, Some(aux::cyberdark::equip_operation(Some(eqfilter_arc()), lua_fn(equipop), true)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop), e1, None, None, None);
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqfilter(c)`: `c:IsLevelBelow(3) and c:IsRace(RACE_DRAGON)` as the filter of `Cyberdark.EquipTarget`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_below(d, c, 3) && card::is_race(d, c, RACE_DRAGON)
}
fn eqfilter_arc() -> CardFilter {
    std::sync::Arc::new(|d: &mut Duel, c: CardId| eqfilter(d, c))
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, a: Args) -> bool {
    // return ec:IsControler(tp) and ec:IsLevelBelow(3) and ec:IsRace(RACE_DRAGON)
    let ec = a.c(0);
    card::is_controler(d, ec, a.p(2)) && card::is_level_below(d, ec, 3) && card::is_race(d, ec, RACE_DRAGON)
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, a: Args) {
    let (c, e, tp, tc) = (a.c(0), a.e(1), a.p(2), a.c(3));
    // local atk=tc:GetTextAttack()
    let mut atk = card::get_text_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_OWNER_RELATE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_OWNER_RELATE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // e2:SetValue(atk)
    effect::set_value(d, e2, atk as i64);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_DESTROY_SUBSTITUTE)
    effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    // e3:SetValue(s.repval)
    effect::set_value_fn(d, e3, lua_fn(repval));
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
}
/// `s.repval(e,re,r,rp)`
fn repval(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    ((a.int(2) as u32) & REASON_BATTLE) != 0
}
