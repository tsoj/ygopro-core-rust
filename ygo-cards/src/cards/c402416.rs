//! Obliterate!!! Blaze
use super::prelude::*;

pub const ID: u32 = 402416;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_EXODIA, SET_FORBIDDEN_ONE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Grant effects to 1 monster you control with "Exodia" in its original name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(10) and c:IsOriginalSetCard(SET_EXODIA) and not c:HasFlagEffect(id)
    card::is_faceup(d, c) && card::is_level_above(d, c, 10) && card::is_original_set_card(d, c, &[SET_EXODIA as u16]) && card::get_flag_effect(d, c, ID) == 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FORBIDDEN_ONE) and c:IsMonster() and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_FORBIDDEN_ONE as u16]) && card::is_monster(d, c) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
            // Duel.HintSelection(tc,true)
            duel::hint_selection(d, tc, true);
            // local c=e:GetHandler()
            let _c = effect::get_handler(d, a.e).nil();
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
            // Destroy cards in the Spell/Trap Zone and equip 5 "Forbidden One" monsters to this card
            // local e1=Effect.CreateEffect(tc)
            let e1 = effect::create_effect(d, tc);
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_EQUIP)
            effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_EQUIP);
            // e1:SetType(EFFECT_TYPE_IGNITION)
            effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetCost(s.descost)
            effect::set_cost(d, e1, Some(cost_target(descost)));
            // e1:SetTarget(s.destg)
            effect::set_target(d, e1, Some(cost_target(destg)));
            // e1:SetOperation(s.desop)
            effect::set_operation(d, e1, Some(operation(desop)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // Inflict piercing damage
            // local e2=Effect.CreateEffect(tc)
            let e2 = effect::create_effect(d, tc);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_PIERCE)
            effect::set_code(d, e2, EFFECT_PIERCE);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_STZONE,LOCATION_STZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_HAND|LOCATION_DECK,0,5,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_STZONE, LOCATION_STZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 5, NO_EX);
    }
    // local sg=Duel.GetMatchingGroup(nil,tp,LOCATION_STZONE,LOCATION_STZONE,nil)
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_STZONE, LOCATION_STZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,tp,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,5,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 5, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sg=Duel.GetMatchingGroup(nil,tp,LOCATION_STZONE,LOCATION_STZONE,nil)
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_STZONE, LOCATION_STZONE, NO_EX);
    // if Duel.Destroy(sg,REASON_EFFECT)>0 and c:IsFaceup() and c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_SZONE)>=5
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_HAND|LOCATION_DECK,0,5,nil,tp) then
    if duel::destroy(d, sg, REASON_EFFECT, None, None) > 0
        && card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, a.e)
        && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 >= 5
        && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 5, NO_EX)
    {
        // local eqg=Duel.GetMatchingGroup(s.eqfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,tp)
        let mut eqg = duel::get_matching_group(d, Some(&|d, x| eqfilter(d, x, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // if #eqg>5 then
        if eqg.len() > 5 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
            // eqg=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_HAND|LOCATION_DECK,0,5,5,nil,tp)
            eqg = duel::select_matching_card(d, a.tp, Some(&|d, x| eqfilter(d, x, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 5, 5, false, NO_EX).nil();
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // for ec in eqg:Iter() do
        for ec in group::iter(d, &eqg) {
            // if Duel.Equip(tp,ec,c,true,true) then
            if duel::equip(d, a.tp, ec, c, true, true) {
                // The equipped monster gains 2000 ATK
                // local e1=Effect.CreateEffect(ec)
                let e1 = effect::create_effect(d, ec);
                // e1:SetType(EFFECT_TYPE_EQUIP)
                effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(2000)
                effect::set_value(d, e1, 2000);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // ec:RegisterEffect(e1)
                card::register_effect(d, ec, e1, false);
                // Equip limit
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetProperty(EFFECT_FLAG_OWNER_RELATE)
                effect::set_property(d, e2, EFFECT_FLAG_OWNER_RELATE, 0);
                // e2:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
                // e2:SetValue(function(e,c) return e:GetOwner()==c end)
                effect::set_value_fn(d, e2, lua_fn(|d, a: Args| effect::get_owner(d, a.e(0)) == Some(a.c(1))));
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // ec:RegisterEffect(e2)
                card::register_effect(d, ec, e2, false);
            }
        }
        // Duel.EquipComplete()
        duel::equip_complete(d);
    }
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c);
    // You cannot activate cards and effects for the rest of this turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(function(_,re) return re:GetHandler()~=c or re:GetHandler():GetFieldID()~=fid end)
    effect::set_value_fn(d, e3, lua_fn(move |d, a: Args| {
        let rh = effect::get_handler(d, a.e(1));
        rh != Some(c) || card::get_field_id(d, c) != fid
    }));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
