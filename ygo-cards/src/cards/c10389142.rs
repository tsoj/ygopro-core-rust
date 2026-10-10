//! Number 42: Galaxy Tomahawk
use super::prelude::*;

pub const ID: u32 = 10389142;
pub const TOKEN_BATTLE_EAGLE: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(42), listed_names: &[TOKEN_BATTLE_EAGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // Special Summon as many "Battle Eagle Tokens" (Machine-Type/WIND/Level 6/ATK 2000/DEF 0) as possible, but destroy them during the End Phase of this turn, also your opponent takes no further battle damage this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_BATTLE_EAGLE,0,TYPES_TOKEN,2000,0,6,RACE_MACHINE,ATTRIBUTE_WIND)`
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_BATTLE_EAGLE,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(2000),
            defense: Some(0),
            level: Some(6),
            race: Some(RACE_MACHINE),
            attribute: Some(ATTRIBUTE_WIND),
        },
        None,
        None,
        0,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `function(ag) Duel.Destroy(ag,REASON_EFFECT) end`
fn destroy_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if ft <= 0 || !can_summon(d, a.tp) {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local sg=Group.CreateGroup()
    let mut sg = CardSet::new();
    // for i=1,ft do
    for _ in 0..ft {
        // local token=Duel.CreateToken(tp,TOKEN_BATTLE_EAGLE)
        let token = duel::create_token(d, a.tp, TOKEN_BATTLE_EAGLE).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // sg:AddCard(token)
        group::add_card(&mut sg, &CardSet::from_card(token));
    }
    // Destroy them during the End Phase of this turn
    // aux.DelayedOperation(sg,PHASE_END,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,nil,1,aux.Stringid(id,1))
    aux::delayed_operation(d, sg, PHASE_END, ID, a.e, a.tp, Some(std::sync::Arc::new(destroy_oper)), None, None, Some(1), Some(aux::stringid(ID, 1)), None);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Your opponent takes no further battle damage this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
