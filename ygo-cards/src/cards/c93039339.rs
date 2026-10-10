//! Super Starslayer TY-PHON - Sky Crisis
use super::prelude::*;

pub const ID: u32 = 93039339;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,12,2,s.xyzfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(12),
        2,
        Some(xyz::alter_filter(xyzfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Neither player can activate the effects of monsters with 3000 or more ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetValue(function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsAttackAbove(3000) end)
    effect::set_value_fn(d, e1, lua_fn(e1_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 monster on the field to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register Special Summons from the Extra Deck
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.xyzfilter(c,tp,xyzc)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, _xyzc: CardId) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, NO_EX);
    // return #g>0 and g:GetMaxGroup(Card.GetAttack,nil):IsContains(c)
    if g.is_empty() {
        return false;
    }
    match group::get_max_group(d, &g, &|d, x| card::get_attack(d, x) as i64) {
        Some((mg, _)) => group::is_contains(&mg, c),
        None => false,
    }
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // for tc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // if tc:IsSummonLocation(LOCATION_EXTRA) then
        if card::is_summon_location(d, tc, LOCATION_EXTRA) {
            // local sp=tc:GetSummonPlayer()
            let sp = card::get_summon_player(d, tc);
            // Duel.RegisterFlagEffect(sp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, sp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
            // if Duel.HasFlagEffect(sp,id,2) then
            if duel::has_flag_effect(d, sp, ID, Some(2)) {
                // Duel.RegisterFlagEffect(sp,id+1,RESET_PHASE|PHASE_END,0,2)
                duel::register_flag_effect(d, sp, ID + 1, RESET_PHASE | PHASE_END, 0, 2, 0);
            }
        }
    }
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // if chk==0 then return Duel.HasFlagEffect(1-tp,id+1) end
    if a.int(2) == 0 {
        return duel::has_flag_effect(d, 1 - tp, ID + 1, None);
    }
    // Cannot Normal or Special Summon this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_OATH | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // return true
    true
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end`
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_xyz_summoned(d, h)
}
/// `function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsAttackAbove(3000) end`
fn e1_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_attack_above(d, h, 3000)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
