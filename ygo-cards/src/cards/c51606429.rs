//! The Phantom Knights of Tomb Shield
//! 幻影騎士団トゥーム・シールド
use super::prelude::*;

pub const ID: u32 = 51606429;

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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_NORMAL,3,0,0,RACE_WARRIOR,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, phantom_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_NORMAL,3,0,0,RACE_WARRIOR,ATTRIBUTE_DARK)`
fn phantom_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_THE_PHANTOM_KNIGHTS as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(3),
        defense: Some(0),
        level: Some(0),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_NORMAL,3,0,0,RACE_WARRIOR,ATTRIBUTE_DARK) then
    if card::is_relate_to_effect(d, c, a.e) && duel::is_player_can_special_summon_monster(d, a.tp, ID, phantom_data(), None, None, 0) {
        // c:AddMonsterAttribute(TYPE_NORMAL)
        card::add_monster_attribute(d, c, TYPE_NORMAL, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP_ATTACK)
        duel::special_summon_step(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP_ATTACK as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.exccon(e) and Duel.IsTurnPlayer(tp)
    aux::exccon().call(d, &[Value::Effect(a.e)]).truthy() && duel::is_turn_player(d, a.tp)
}
/// `s.negfilter(c)`
fn negfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsTrap() and not c:IsDisabled()
    card::is_faceup(d, c) && card::is_trap(d, c) && !card::is_disabled(d, c)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and s.negfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && negfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.negfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&negfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // Duel.SelectTarget(tp,s.negfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&negfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsDisabled() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_disabled(d, tc) {
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // if tc:IsType(TYPE_TRAPMONSTER) then
            if card::is_type(d, tc, TYPE_TRAPMONSTER) {
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
                effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
                // e3:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
                // tc:RegisterEffect(e3)
                card::register_effect(d, tc, e3, false);
            }
        }
    }
}
