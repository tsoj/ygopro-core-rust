//! Spright Pixies
use super::prelude::*;

pub const ID: u32 = 49928686;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon self
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Boost ATK/DEF of battling monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE | CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsLevel(2) or c:IsRank(2))
    card::is_faceup(d, c) && (card::is_level(d, c, &[2]) || card::is_rank(d, c, &[2]))
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,d=Duel.GetBattleMonster(tp)
    let (bat, bdef) = duel::get_battle_monster(d, a.tp);
    // return a and d and a:IsFaceup() and d:IsFaceup()
    //     and a~=e:GetHandler() and (a:IsLevel(2) or a:IsRank(2) or a:IsLink(2))
    let (Some(bat), Some(bdef)) = (bat, bdef) else {
        return false;
    };
    card::is_faceup(d, bat)
        && card::is_faceup(d, bdef)
        && Some(bat) != effect::get_handler(d, a.e)
        && (card::is_level(d, bat, &[2]) || card::is_rank(d, bat, &[2]) || card::is_link(d, bat, &[2]))
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local a,d=Duel.GetBattleMonster(tp)
    let (bat, bdef) = duel::get_battle_monster(d, a.tp);
    // if a and d and a:IsRelateToBattle() and a:IsFaceup() and d:IsRelateToBattle() and d:IsFaceup()
    //     and d:IsControler(1-tp) then
    let (Some(bat), Some(bdef)) = (bat, bdef) else {
        return;
    };
    if card::is_relate_to_battle(d, bat)
        && card::is_faceup(d, bat)
        && card::is_relate_to_battle(d, bdef)
        && card::is_faceup(d, bdef)
        && card::is_controler(d, bdef, 1 - a.tp)
    {
        // Increase ATK/DEF
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(d:GetAttack())
        let datk = card::get_attack(d, bdef) as i64;
        effect::set_value(d, e1, datk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // a:RegisterEffect(e1)
        card::register_effect(d, bat, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // a:RegisterEffect(e2)
        card::register_effect(d, bat, e2, false);
    }
}
