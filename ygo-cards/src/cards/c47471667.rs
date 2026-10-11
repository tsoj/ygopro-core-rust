//! D - Burst
use super::prelude::*;

pub const ID: u32 = 47471667;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[17132130], listed_series: &[SET_DESTINY_HERO as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 face-up Spell you control; destroy it, and if you do, draw 1 card, then if you control a "Destiny HERO" monster, you can Special Summon 1 "Destiny HERO" monster from your hand, GY, or banishment
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // At the end of the Damage Step, if a monster equipped with an Equip Card, or "Destiny HERO - Dogma", attacked: You can banish this card from your GY; that monster can make a second attack in a row
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(function() Duel.ChainAttack() end)
    effect::set_operation(d, e2, Some(operation(chain_attack_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsOnField() and chkc:IsSpell() and chkc:IsFaceup() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_on_field(d, chkc) && card::is_spell(d, chkc) && card::is_faceup(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSpell),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&aux::faceup_filter(|d, x| card::is_spell(d, x))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSpell),tp,LOCATION_ONFIELD,0,1,1,c)
    let g = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, x| card::is_spell(d, x))), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DESTINY_HERO) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DESTINY_HERO as u16])
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and Duel.Draw(tp,1,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE,0)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DESTINY_HERO),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::draw(d, tp, 1, REASON_EFFECT) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(0), None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_DESTINY_HERO as u16]))),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
            0,
            1,
            NO_EX,
        )
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d).nil();
    // return (ac:HasEquipCard() or ac:IsCode(17132130)) and ac:CanChainAttack()
    (card::has_equip_card(d, ac) || card::is_code(d, ac, &[17132130])) && card::can_chain_attack(d, ac, None, false)
}
// function() Duel.ChainAttack() end
fn chain_attack_op(d: &mut Duel, _a: &EffArgs) {
    duel::chain_attack(d, None);
}
