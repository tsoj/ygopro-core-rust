//! Cyberdark Keel
use super::prelude::*;

pub const ID: u32 = 3019642;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBERDARK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip
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
    // Damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCondition(aux.bdocon)
    effect::set_condition(d, e2, Some(aux::bdocon()));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn eqfilter_arc() -> CardFilter {
    std::sync::Arc::new(|d: &mut Duel, c: CardId| eqfilter(d, c))
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(3) and c:IsRace(RACE_DRAGON)
    card::is_level_below(d, c, 3) && card::is_race(d, c, RACE_DRAGON)
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, a: Args) -> bool {
    // return ec:IsControler(tp) and ec:IsLevelBelow(3) and ec:IsRace(RACE_DRAGON)
    card::is_controler(d, a.c(0), a.p(2)) && card::is_level_below(d, a.c(0), 3) && card::is_race(d, a.c(0), RACE_DRAGON)
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
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(300)
    duel::set_target_param(d, 300);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 300);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int() as i64, REASON_EFFECT, false, None);
}
