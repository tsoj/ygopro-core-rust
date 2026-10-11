//! Scareclaw Straddle
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 41619242;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], listed_series: &[SET_SCARECLAW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Target 1 "Scareclaw" monster or 1 "Visas Starfrost" in your Monster Zone, and 1 face-up monster your opponent controls; your monster gains ATK/DEF equal to your opponent's monster's ATK or DEF (whichever is higher)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END|TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END | TIMING_DAMAGE_STEP as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --When a card or effect is activated that targets a "Scareclaw" monster or "Visas Starfrost" you control: Negate that effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end)
    effect::set_operation(
        d,
        e2,
        Some(operation(|d, a| {
            duel::negate_effect(d, a.ev as u8);
        })),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atkfilter(c,opp)
fn atkfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return ((c:IsSetCard(SET_SCARECLAW) or c:IsCode(CARD_VISAS_STARFROST)) or (c:IsControler(opp)
    //     and (c:HasNonZeroAttack() or c:HasNonZeroDefense()))) and c:IsFaceup()
    ((card::is_set_card(d, c, &[SET_SCARECLAW as u16]) || card::is_code(d, c, &[CARD_VISAS_STARFROST]))
        || (card::is_controler(d, c, opp) && (card::has_non_zero_attack(d, c) || card::has_non_zero_defense(d, c))))
        && card::is_faceup(d, c)
}
/// `aux.dpcheck(Card.GetControler)`
fn controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,1-tp)
    let g = duel::get_target_group(d, Some(&|d, c| atkfilter(d, c, 1 - tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let dp = aux::dpcheck(controler_value);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_ATKDEF)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_ATKDEF as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, &tg);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local tc,ac=g:GetFirst(),g:GetNext()
    let order = group::iter(d, &g);
    let (mut tc, mut ac) = (order[0], order[1]);
    // if tc:IsControler(1-tp) then tc,ac=ac,tc end
    if card::is_controler(d, tc, 1 - tp) {
        std::mem::swap(&mut tc, &mut ac);
    }
    // if tc:IsFaceup() and tc:IsControler(tp) and ac:IsFaceup() and ac:IsControler(1-tp) then
    if card::is_faceup(d, tc) && card::is_controler(d, tc, tp) && card::is_faceup(d, ac) && card::is_controler(d, ac, 1 - tp) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local val=math.max(ac:GetAttack(),ac:GetDefense())
        let val = card::get_attack(d, ac).max(card::get_defense(d, ac)) as i64;
        // tc:UpdateAttack(val,nil,c)
        card::update_attack(d, tc, val, None, Some(c), None);
        // tc:UpdateDefense(val,nil,c)
        card::update_defense(d, tc, val, None, Some(c), None);
    }
}
// function s.disconfilter(c,tp)
fn disconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return ((c:IsSetCard(SET_SCARECLAW) and c:IsMonster()) or c:IsCode(CARD_VISAS_STARFROST))
    //     and c:IsFaceup() and c:IsOnField() and c:IsControler(tp)
    ((card::is_set_card(d, c, &[SET_SCARECLAW as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[CARD_VISAS_STARFROST]))
        && card::is_faceup(d, c)
        && card::is_on_field(d, c)
        && card::is_controler(d, c, tp)
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    // if not (re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and Duel.IsChainDisablable(ev)) then return false end
    if !(effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) && duel::is_chain_disablable(d, a.ev as u8)) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // return tg and tg:IsExists(s.disconfilter,1,nil,tp)
    match tg {
        Some(tg) => group::is_exists(d, &tg, &|d, c| disconfilter(d, c, tp), 1, NO_EX),
        None => false,
    }
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    true
}
