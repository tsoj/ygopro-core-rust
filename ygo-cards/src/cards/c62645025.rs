//! The Phantom Knights of Wrong Magnetring
//! 幻影騎士団ウロング・マグネリング
use super::prelude::*;

pub const ID: u32 = 62645025;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_PHANTOM_KNIGHTS, SET_PHANTOM_KNIGHTS], ..ScriptInfo::DEFAULT };

/// The Lua `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_EFFECT,0,0,2,RACE_WARRIOR,ATTRIBUTE_DARK)`
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_THE_PHANTOM_KNIGHTS as u16]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(0),
        defense: Some(0),
        level: Some(2),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_DARK),
    }
}

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
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetCost(s.drcost)
    effect::set_cost(d, e2, Some(cost_target(drcost)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp)
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_EFFECT,0,0,2,RACE_WARRIOR,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.NegateAttack() then return end
    if !duel::negate_attack(d) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e)
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_EFFECT,0,0,2,RACE_WARRIOR,ATTRIBUTE_DARK) then return end
    if !card::is_relate_to_effect(d, c, a.e) || !duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT)
    card::add_monster_attribute(d, c, TYPE_EFFECT, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP_ATTACK)
    duel::special_summon_step(d, c, 1, tp, tp, true, false, POS_FACEUP_ATTACK as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // if c:IsFacedown() or not c:IsAbleToGraveAsCost() then return false end
    if card::is_facedown(d, c) || !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    // return (c:IsSetCard(SET_THE_PHANTOM_KNIGHTS) and c:IsMonster())
    //     or (c:IsSetCard(SET_PHANTOM_KNIGHTS) and c:IsSpellTrap() and c:IsType(TYPE_CONTINUOUS))
    (card::is_set_card(d, c, &[SET_THE_PHANTOM_KNIGHTS as u16]) && card::is_monster(d, c))
        || (card::is_set_card(d, c, &[SET_PHANTOM_KNIGHTS as u16]) && card::is_spell_trap(d, c) && card::is_type(d, c, TYPE_CONTINUOUS))
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_ONFIELD,0,1,1,c)
    let mut g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
