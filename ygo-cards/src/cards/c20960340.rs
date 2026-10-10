//! Krystal Avatar
use super::prelude::*;

pub const ID: u32 = 20960340;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    //     and Duel.GetAttacker():IsAttackAbove(Duel.GetLP(tp))
    let at = duel::get_attacker(d).nil();
    if !card::is_controler(d, at, 1 - tp) {
        return false;
    }
    if duel::get_attack_target(d).is_some() {
        return false;
    }
    let at = duel::get_attacker(d).nil();
    let lp = duel::get_lp(d, tp);
    card::is_attack_above(d, at, lp)
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,atk,0,4,RACE_WARRIOR,ATTRIBUTE_LIGHT)`
fn can_summon(d: &mut Duel, tp: u8, atk: i32) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPE_MONSTER | TYPE_EFFECT),
            attack: Some(atk),
            defense: Some(0),
            level: Some(4),
            race: Some(RACE_WARRIOR),
            attribute: Some(ATTRIBUTE_LIGHT),
        },
        None,
        None,
        0,
    )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local atk=Duel.GetLP(tp)
    let atk = duel::get_lp(d, tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, tp, atk);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local atk=Duel.GetLP(tp)
    let atk = duel::get_lp(d, tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon(d, tp, atk) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_TRAP|TYPE_EFFECT)
    card::add_monster_attribute(d, c, TYPE_TRAP | TYPE_EFFECT, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP_ATTACK)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP_ATTACK as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(atk)
    effect::set_value(d, e1, atk as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
    // c:RegisterEffect(e2,true)
    card::register_effect(d, c, e2, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // local at=Duel.GetAttacker()
    // if at and at:CanAttack() and at:IsFaceup() and not at:IsImmuneToEffect(e) and not at:IsStatus(STATUS_ATTACK_CANCELED) then
    if let Some(at) = duel::get_attacker(d) {
        if card::can_attack(d, at) && card::is_faceup(d, at) && !card::is_immune_to_effect(d, at, a.e) && !card::is_status(d, at, STATUS_ATTACK_CANCELED) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ChangeAttackTarget(c)
            duel::change_attack_target(d, Some(c), false);
        }
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, a.e).nil();
    card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dam=e:GetHandler():GetAttack()
    let h = effect::get_handler(d, a.e).nil();
    let dam = card::get_attack(d, h);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Damage(p,e:GetHandler():GetAttack(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    let amount = card::get_attack(d, h);
    duel::damage(d, p, amount as i64, REASON_EFFECT, false, None);
}
