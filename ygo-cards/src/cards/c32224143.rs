//! Ghostrick Socuteboss
use super::prelude::*;

pub const ID: u32 = 32224143;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,2,2)
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(lua_fn(atkcon)));
    // e2:SetValue(aux.imval2)
    effect::set_value_fn(d, e2, aux::imval2());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GHOSTRICK as u16])
}
// function s.filter(c,atk)
fn filter(d: &mut Duel, c: CardId, atk: i64) -> bool {
    card::is_faceup(d, c) && card::is_attack_below(d, c, atk as i32)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let cg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local atk=cg:GetSum(Card.GetAttack)
    let atk = group::get_sum(d, &cg, &|d, c| card::get_attack(d, c) as i64);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,atk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,atk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| filter(d, c, atk)),
            a.tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,atk)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| filter(d, c, atk)),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local seq=tc:GetSequence()
    let mut seq = card::get_sequence(d, tc) as i64;
    // if tc:IsControler(1-tp) then seq=seq+16 end
    if card::is_controler(d, tc, 1 - a.tp) {
        seq += 16;
    }
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) != 0
    {
        // local g=Duel.GetMatchingGroup(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
        let hp = effect::get_handler_player(d, a.e);
        let g = duel::get_matching_group(d, Some(&cfilter), hp, LOCATION_MZONE, 0, NO_EX);
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        //     tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_OVERLAY,0,1)
        // end
        for tc2 in group::iter(d, &g) {
            card::register_flag_effect(d, tc2, ID, RESET_EVENT | RESETS_STANDARD | RESET_OVERLAY, 0, 1, 0, 0);
        }
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_DISABLE_FIELD)
        effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
        // e1:SetLabel(seq)
        effect::set_label(d, e1, &[seq]);
        // e1:SetCondition(s.discon)
        effect::set_condition(d, e1, Some(lua_fn(discon)));
        // e1:SetOperation(s.disop)
        effect::set_operation(d, e1, Some(lua_fn(disop)));
        // e1:SetReset(0)
        effect::set_reset(d, e1, 0, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// function s.cfilter2(c)
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return s.cfilter(c) and c:GetFlagEffect(id)~=0
    cfilter(d, c) && card::get_flag_effect(d, c, ID) != 0
}
// function s.discon(e)
fn discon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local g=Duel.GetMatchingGroup(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
    let hp = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&cfilter), hp, LOCATION_MZONE, 0, NO_EX);
    // if g:IsExists(s.cfilter2,1,nil) then
    if group::is_exists(d, &g, &|d, c| cfilter2(d, c), 1, NO_EX) {
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        //     tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_OVERLAY,0,1)
        //     tc=g:GetNext()
        // end
        // aux.Next calls g:GetFirst() then g:GetNext() on the group's cursor, and the body
        // calls g:GetNext() once more: every processed card moves the cursor one extra step.
        let order = group::iter(d, &g);
        let mut cur = 0usize;
        let mut first = true;
        loop {
            if !first {
                cur += 1;
            }
            first = false;
            let Some(&tc) = order.get(cur) else { break };
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD | RESET_OVERLAY, 0, 1, 0, 0);
            // tc=g:GetNext()
            cur += 1;
        }
        // return true
        return true;
    }
    // e:Reset()
    effect::reset(d, e);
    // return false
    false
}
// function s.disop(e,tp) return (0x1<<e:GetLabel()) end
fn disop(d: &mut Duel, a: Args) -> i64 {
    1i64 << effect::get_label(d, a.e(0))
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, h.into())
}
