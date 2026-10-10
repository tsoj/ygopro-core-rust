//! Divine Arsenal AA-ZEUS - Sky Thunder
use super::prelude::*;

pub const ID: u32 = 90448279;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
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
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Send cards to GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // e1:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e1:SetTarget(s.gytg)
    effect::set_target(d, e1, Some(cost_target(gytg)));
    // e1:SetOperation(s.gyop)
    effect::set_operation(d, e1, Some(operation(gyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 card to itself
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.attcon)
    effect::set_condition(d, e2, Some(condition(attcon)));
    // e2:SetTarget(s.atttg)
    effect::set_target(d, e2, Some(cost_target(atttg)));
    // e2:SetOperation(s.attop)
    effect::set_operation(d, e2, Some(operation(attop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLED)
        effect::set_code(d, ge1, EVENT_BATTLED);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let def = duel::get_attack_target(d);
    // if (a and a:IsType(TYPE_XYZ)) or (d and d:IsType(TYPE_XYZ)) then
    let atk_xyz = match atk {
        Some(x) => card::is_type(d, x, TYPE_XYZ),
        None => false,
    };
    let def_xyz = match def {
        Some(x) => card::is_type(d, x, TYPE_XYZ),
        None => false,
    };
    if atk_xyz || def_xyz {
        // Duel.RegisterFlagEffect(0,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 0, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.xyzfilter(c,tp,xyzc)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c) && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 and Duel.GetFlagEffect(0,id+1)>0 end
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0 && duel::get_flag_effect(d, 0, ID + 1) > 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // return true
    true
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
/// `s.attconfilter(c,tp)`
fn attconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp)) and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD)
    (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.attcon(e,tp,eg,ep,ev,re,r,rp)`
fn attcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.attconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| attconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.attfilter(c,e)`
fn attfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return not c:IsImmuneToEffect(e)
    !card::is_immune_to_effect(d, c, e)
}
/// `s.atttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e)
    //     and e:GetHandler():IsType(TYPE_XYZ) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| attfilter(d, c, a.e)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
            && card::is_type(d, effect::get_handler(d, a.e).nil(), TYPE_XYZ);
    }
    true
}
/// `s.attop(e,tp,eg,ep,ev,re,r,rp)`
fn attop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.attfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| attfilter(d, x, a.e)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Overlay(c,tc,true)
        duel::overlay(d, c, tc, true);
    }
}
