//! Endless Bond
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 35989913;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[84013237], listed_series: &[SET_XYZ, SET_UTOPIA, SET_UTOPIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --When this card is activated: You can Special Summon 1 "Number 39: Utopia" from your GY, and if you do, it gains ATK equal to the total ATK of all LIGHT "Utopia" and "Utopic" Xyz Monsters in your GY
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Make that monster you control lose 1000 ATK, and if you do, it can attack again in a row
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(84013237) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[84013237]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.utopigyfilter(c)`
fn utopigyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsSetCard({SET_UTOPIA,SET_UTOPIC}) and c:IsXyzMonster()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_set_card(d, c, &[SET_UTOPIA as u16, SET_UTOPIC as u16]) && card::is_xyz_monster(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // local atk=Duel.GetMatchingGroup(s.utopigyfilter,tp,LOCATION_GRAVE,0,nil):GetSum(Card.GetAttack)
                let gy = duel::get_matching_group(d, Some(&utopigyfilter), tp, LOCATION_GRAVE, 0, NO_EX);
                let atk = group::get_sum(d, &gy, &|d, x| card::get_attack(d, x) as i64);
                // sc:UpdateAttack(atk,RESET_EVENT|RESETS_STANDARD,e:GetHandler())
                let c = effect::get_handler(d, e).nil();
                card::update_attack(d, sc, atk, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
            }
        }
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local bc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let bc = group::get_first(d, &eg);
    // return bc==Duel.GetAttacker() and bc:IsControler(tp) and bc:CanChainAttack() and bc:IsSetCard({SET_UTOPIA,SET_UTOPIC})
    //   and bc:IsOriginalAttribute(ATTRIBUTE_LIGHT) and bc:GetBattleTarget():IsPreviousControler(1-tp)
    if bc != duel::get_attacker(d) {
        return false;
    }
    let bc = bc.nil();
    card::is_controler(d, bc, tp)
        && card::can_chain_attack(d, bc, None, false)
        && card::is_set_card(d, bc, &[SET_UTOPIA as u16, SET_UTOPIC as u16])
        && card::is_original_attribute(d, bc, ATTRIBUTE_LIGHT)
        && card::is_previous_controler(d, card::get_battle_target(d, bc).nil(), 1 - tp)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let bc = group::get_first(d, &eg).nil();
    // e:SetLabelObject(bc)
    effect::set_label_object(d, a.e, Value::Card(bc));
    // bc:CreateEffectRelation(e)
    card::create_effect_relation(d, bc, a.e);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, e).as_card().nil();
    // if bc:IsRelateToEffect(e) and bc:IsControler(tp) and bc:IsFaceup() and not bc:IsImmuneToEffect(e)
    //   and bc:UpdateAttack(-1000,RESET_EVENT|RESETS_STANDARD,e:GetHandler())<0 then
    if card::is_relate_to_effect(d, bc, e)
        && card::is_controler(d, bc, tp)
        && card::is_faceup(d, bc)
        && !card::is_immune_to_effect(d, bc, e)
        && {
            let c = effect::get_handler(d, e).nil();
            card::update_attack(d, bc, -1000, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None) < 0
        }
    {
        // Duel.ChainAttack()
        duel::chain_attack(d, None);
    }
}
