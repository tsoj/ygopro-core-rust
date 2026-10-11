//! Zoma the Earthbound Spirit
use super::prelude::*;

pub const ID: u32 = 88001391;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_END_PHASE));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Inflict damage equal to double the original ATK of the monster that destroyed it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1800,500,4,RACE_ZOMBIE,ATTRIBUTE_DARK,POS_FACEUP,tp,1)`
fn can_summon(d: &mut Duel, tp: u8, typ: u32) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(typ),
            attack: Some(1800),
            defense: Some(500),
            level: Some(4),
            race: Some(RACE_ZOMBIE),
            attribute: Some(ATTRIBUTE_DARK),
        },
        Some(POS_FACEUP as u8),
        Some(tp),
        1,
    )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1800,500,4,RACE_ZOMBIE,ATTRIBUTE_DARK,POS_FACEUP,tp,1) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, tp, TYPE_MONSTER | TYPE_EFFECT);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(h.into()), 1, tp, 0);
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
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon(d, tp, TYPE_MONSTER + TYPE_EFFECT) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 1, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Monsters the opponent controls must attack this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
    // e2:SetValue(function(e,ac) return ac==e:GetHandler() end)
    effect::set_value_fn(d, e2, lua_fn(must_attack_value));
    // c:RegisterEffect(e2,true)
    card::register_effect(d, c, e2, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `function(e,ac) return ac==e:GetHandler() end`
fn must_attack_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let ac = a.card(1);
    let h = effect::get_handler(d, e).nil();
    ac == Some(h)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d);
    // if chk==0 then return c:IsSummonType(1) and ac==c:GetBattleTarget() and ac:IsControler(1-tp) end
    if a.chk == Some(0) {
        if !card::is_summon_type(d, c, &[1]) {
            return false;
        }
        if ac != card::get_battle_target(d, c) {
            return false;
        }
        return card::is_controler(d, ac.nil(), 1 - tp);
    }
    // local dam=math.min(3000,ac:GetBaseAttack()*2)
    let dam = (card::get_base_attack(d, ac.nil()) * 2).min(3000);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // local dam=math.min(3000,bc:GetBaseAttack()*2)
    let dam = (card::get_base_attack(d, bc.nil()) * 2).min(3000);
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Damage(p,dam,REASON_EFFECT)
    duel::damage(d, p, dam as i64, REASON_EFFECT, false, None);
}
