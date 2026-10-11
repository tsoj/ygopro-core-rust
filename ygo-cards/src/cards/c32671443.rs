//! Doctor D
use super::prelude::*;

pub const ID: u32 = 32671443;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DESTINY_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to your hand or Special Summon 1 "Destiny HERO" monster from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make the ATK of a "Destiny Hero" monster becomes the ATK of another "Destiny Hero"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.costfilter(c,e,tp,ft)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsSetCard(SET_DESTINY_HERO) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,c,e,tp,ft)
    card::is_set_card(d, c, &[SET_DESTINY_HERO as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, e, tp, ft)), tp, LOCATION_GRAVE, 0, 1, c.into())
}
/// `s.filter(c,e,tp,ft)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return (c:IsSetCard(SET_DESTINY_HERO) and c:IsMonster()) and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    (card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_monster(d, c))
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, a.e, tp, ft)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp,ft)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, a.e, tp, ft)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp,ft):GetFirst()
    let nv = aux::necro_valley_filter(|d, c| filter(d, c, a.e, tp, ft));
    let sel = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sel) {
        // aux.ToHandOrElse(tc,tp,function(c)
        //     return tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and ft>0 end,
        // function(c)
        //     Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
        // aux.Stringid(id,2))
        let e = a.e;
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, _c| card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None) && ft > 0),
            Some(&|d, _g| {
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 2)),
        );
    }
}
/// `s.atkfilter1(c,tp)`
fn atkfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DESTINY_HERO) and c:IsMonster()
    //     and Duel.IsExistingTarget(s.atkfilter2,tp,LOCATION_MZONE,0,1,c,c:GetAttack())
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_monster(d, c)) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_target(d, Some(&|d, x| atkfilter2(d, x, atk)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.atkfilter2(c,atk)`
fn atkfilter2(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DESTINY_HERO) and c:IsMonster() and not c:IsAttack(atk)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_monster(d, c) && !card::is_attack(d, c, &[atk as u32])
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| atkfilter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local g1=Duel.SelectTarget(tp,s.atkfilter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| atkfilter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,s.atkfilter2,tp,LOCATION_MZONE,0,1,1,g1,g1:GetFirst():GetAttack())
    let first = group::get_first(d, &g1).nil();
    let atk = card::get_attack(d, first);
    duel::select_target(d, a.tp, Some(&|d, c| atkfilter2(d, c, atk)), a.tp, LOCATION_MZONE, 0, 1, 1, false, (&g1).into());
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let targets = duel::get_target_cards(d, a.e).nil();
    let g = group::filter(d, &targets, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    // local tg=g:Select(tp,1,1,nil)
    let tg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(tg)
    duel::hint_selection(d, tg.clone(), true);
    // local hc=tg:GetFirst()
    let hc = group::get_first(d, &tg);
    // local tc=(g-hc):GetFirst()
    let mut rest = g.clone();
    group::remove_card(&mut rest, &CardSet::from_card(hc.nil()));
    let tc = group::get_first(d, &rest);
    // if hc and tc then
    if let (Some(hc), Some(tc)) = (hc, tc) {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // Change ATK
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // hc:RegisterEffect(e1)
        card::register_effect(d, hc, e1, false);
    }
}
