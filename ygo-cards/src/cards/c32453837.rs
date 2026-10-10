//! Number 2: Ninja Shadow Mosquito
use super::prelude::*;

pub const ID: u32 = 32453837;

pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(2),
    counter_place_list: &[0x1101],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,2,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Avoid battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Monsters the opponent controls must attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e3, EFFECT_MUST_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // When a monster declares attack, place 1 Hallucination Counter or deal damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.ctdmgtg)
    effect::set_target(d, e4, Some(cost_target(ctdmgtg)));
    // e4:SetOperation(s.ctdmgop)
    effect::set_operation(d, e4, Some(operation(ctdmgop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.hcounter(c)`
fn hcounter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetCounter(0x1101)>0 and c:GetAttack()>0
    card::is_faceup(d, c) && card::get_counter(d, c, 0x1101) > 0 && card::get_attack(d, c) > 0
}
/// `s.ctdmgtg(e,tp,...,chk)`
fn ctdmgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
    //   and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    let b1 = card::check_remove_overlay_card(d, h, tp, 1, REASON_EFFECT)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.hcounter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&hcounter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,0)},{b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_COUNTER)
        effect::set_category(d, a.e, CATEGORY_COUNTER);
        // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,tp,0x1101)
        duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, tp, 0x1101);
    } else {
        // e:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_DAMAGE);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
    }
    true
}
/// `function(e) return e:GetHandler():GetCounter(0x1101)>0 end` (the Disable effect's condition)
fn negate_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_counter(d, h, 0x1101) > 0
}
/// `s.ctdmgop(e,tp,...)`
fn ctdmgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0 then
        if card::is_relate_to_effect(d, c, a.e) && card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT) > 0 {
            // local cg=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
            let cg = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, NO_EX);
            // if #cg==0 then return end
            if cg.is_empty() {
                return;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_COUNTER as u64);
            // local tc=cg:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &cg, tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // Duel.HintSelection(tc,true)
            duel::hint_selection(d, tc, true);
            // if tc:AddCounter(0x1101,1) then
            if card::add_counter(d, tc, 0x1101, 1, false) {
                // Negate its effects
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetCondition(function(e) return e:GetHandler():GetCounter(0x1101)>0 end)
                effect::set_condition(d, e1, Some(lua_fn(negate_cond)));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    } else {
        // Deal damage
        // local g=Duel.GetMatchingGroup(s.hcounter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&hcounter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // Duel.Damage(1-tp,tc:GetAttack(),REASON_EFFECT)
        let amount = card::get_attack(d, tc) as i64;
        duel::damage(d, 1 - tp, amount, REASON_EFFECT, false, None);
    }
}
