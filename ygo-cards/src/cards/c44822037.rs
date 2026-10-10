//! Angel Statue - Azurune
use super::prelude::*;

pub const ID: u32 = 44822037;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetTarget(s.bttg)
    effect::set_target(d, e2, Some(cost_target(bttg)));
    // e2:SetOperation(s.btop)
    effect::set_operation(d, e2, Some(operation(btop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1800,1800,4,RACE_FAIRY,ATTRIBUTE_LIGHT)`
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPE_MONSTER | TYPE_EFFECT),
            attack: Some(1800),
            defense: Some(1800),
            level: Some(4),
            race: Some(RACE_FAIRY),
            attribute: Some(ATTRIBUTE_LIGHT),
        },
        None,
        None,
        0,
    )
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon(d, a.tp) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // --Negate the Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1, EVENT_SPSUMMON);
    // e1:SetCondition(s.ngcon)
    effect::set_condition(d, e1, Some(condition(ngcon)));
    // e1:SetCost(s.ngcost)
    effect::set_cost(d, e1, Some(cost_target(ngcost)));
    // e1:SetTarget(s.ngtg)
    effect::set_target(d, e1, Some(cost_target(ngtg)));
    // e1:SetOperation(s.ngop)
    effect::set_operation(d, e1, Some(operation(ngop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.ngcon(e,tp,eg,ep,...)`
fn ngcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain()==0 and tp~=ep
    duel::get_current_chain(d, false) == 0 && a.tp != a.ep
}
/// `s.ngfilter(c)`
fn ngfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_SZONE) and c:IsContinuousTrap() and c:IsAbleToGraveAsCost()
    card::is_summon_location(d, c, LOCATION_SZONE) && card::is_continuous_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.ngcost(e,tp,...,chk)`
fn ngcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ngfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ngfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.ngfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&ngfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.ngtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ngtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), eg.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
/// `s.ngop(e,tp,eg,...)`
fn ngop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, eg.clone());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, eg, REASON_EFFECT, None, None);
}
/// `s.bttg(e,tp,...,chk)`
fn bttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if chk==0 then return bc:IsRelateToBattle() end
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, bc.nil());
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.btop(e,tp,...)`
fn btop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // Duel.Destroy(bc,REASON_EFFECT)
        duel::destroy(d, bc, REASON_EFFECT, None, None);
    }
}
