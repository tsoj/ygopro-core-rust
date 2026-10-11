//! Flower Cardian Cherry Blossom with Curtain
use super::prelude::*;

pub const ID: u32 = 5489987;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, true, false, None, None, None);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,c,2,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 2, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // if Duel.Draw(p,d,REASON_EFFECT)>0 then
    if duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT) > 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local dc=Duel.GetOperatedGroup():GetFirst()
        let operated = duel::get_operated_group(d);
        let dc = group::get_first(d, &operated).nil();
        // Duel.ConfirmCards(1-tp,dc)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(dc));
        // if dc:IsSetCard(SET_FLOWER_CARDIAN) and dc:IsMonster() then
        if card::is_set_card(d, dc, &[SET_FLOWER_CARDIAN as u16]) && card::is_monster(d, dc) {
            // if c:IsRelateToEffect(e) then
            if card::is_relate_to_effect(d, c, a.e) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // if Duel.SpecialSummon(c,0,tp,tp,true,false,POS_FACEUP)>0 then
                if duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
                    // c:CompleteProcedure()
                    card::complete_procedure(d, c);
                }
            }
        } else {
            // local g=Group.FromCards(dc)
            let mut g = CardSet::from_card(dc);
            // if c:IsRelateToEffect(e) then g:AddCard(c) end
            if card::is_relate_to_effect(d, c, a.e) {
                group::add_card(&mut g, &CardSet::from_card(c));
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not aux.StatChangeDamageStepCondition() then return false end
    if !aux::stat_change_damage_step_condition(d) {
        return false;
    }
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    let (bc1, bc2) = duel::get_battle_monster(d, a.tp);
    // return bc1 and bc2 and bc1:IsSetCard(SET_FLOWER_CARDIAN)
    match (bc1, bc2) {
        (Some(bc1), Some(_)) => card::is_set_card(d, bc1, &[SET_FLOWER_CARDIAN as u16]),
        _ => false,
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    // e:SetLabelObject(bc)
    let v = match bc {
        Some(bc) => Value::Card(bc),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, v);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetLabelObject()
    let bc = match effect::get_label_object(d, a.e) {
        Value::Card(bc) => bc,
        _ => crate::lua_error("attempt to index a nil value"),
    };
    // if bc:IsRelateToBattle() and bc:IsFaceup() and bc:IsControler(tp) then
    if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) && card::is_controler(d, bc, a.tp) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
}
