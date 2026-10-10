//! Successor Soul
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 69145169;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, move |d| {
        // Check attacking monster
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
        // e2:SetOperation(s.checkop)
        effect::set_operation(d, e2, Some(operation(checkop)));
        // Duel.RegisterEffect(e2,0)
        duel::register_effect(d, e2, 0);
    });
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsAbleToGrave() and (not e or c:IsCanBeEffectTarget(e))
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EFFECT)
        && card::is_able_to_grave(d, c, None)
        && e.map_or(true, |e| card::is_can_be_effect_target(d, c, Some(e)))
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsType(TYPE_EFFECT)
    card::is_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,e)
    let dg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, Some(a.e))), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,aux.ReleaseCheckTarget,nil,dg) and
    //   (Duel.GetFlagEffect(tp,id)==0 or Duel.GetFlagEffectLabel(tp,id)~=0) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(
            d,
            a.tp,
            Some(&cfilter),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
            NO_EX,
        ) && (duel::get_flag_effect(d, a.tp, ID) == 0 || duel::get_flag_effect_label(d, a.tp, ID).first().copied() != Some(0));
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,aux.ReleaseCheckTarget,nil,dg)
    let g = duel::select_release_group_cost(
        d,
        a.tp,
        Some(&cfilter),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg)),
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // Register can only attack with 1 monster
    // Prevents other attacks
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(lua_fn(atkcon)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetReset(RESET_PHASE|PHASE_BATTLE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    let h2 = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, h2, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsLevelAbove(7) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_NORMAL) && card::is_level_above(d, c, 7) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, Some(a.e));
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,e)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, Some(a.e))), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,e)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, Some(a.e))), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoGrave(tc,REASON_EFFECT)>0
    //   and Duel.GetOperatedGroup():GetFirst():IsLocation(LOCATION_GRAVE) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0 {
            let og = duel::get_operated_group(d);
            let f = group::get_first(d, &og).nil();
            if card::is_location(d, f, LOCATION_GRAVE) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
                let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                // if #g>0 then
                if !g.is_empty() {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)~=0
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_flag_effect(d, p, ID) != 0
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetFieldID()~=Duel.GetFlagEffectLabel(e:GetHandlerPlayer(),id)
    let p = effect::get_handler_player(d, e);
    let label = duel::get_flag_effect_label(d, p, ID).first().copied();
    Some(card::get_field_id(d, c) as i64) != label
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local fid=eg:GetFirst():GetFieldID()
    let first = group::get_first(d, &a.eg.clone().nil()).nil();
    let fid = card::get_field_id(d, first);
    // local p=eg:GetFirst():GetControler()
    let first2 = group::get_first(d, &a.eg.clone().nil()).nil();
    let p = card::get_controler(d, first2);
    // if Duel.GetFlagEffect(p,id)~=0 and Duel.GetFlagEffectLabel(p,id)~=fid then
    if duel::get_flag_effect(d, p, ID) != 0 && duel::get_flag_effect_label(d, p, ID).first().copied() != Some(fid as i64) {
        // Duel.SetFlagEffectLabel(p,id,0)
        duel::set_flag_effect_label(d, p, ID, 0);
    } else {
        // Duel.RegisterFlagEffect(p,id,RESETS_STANDARD_PHASE_END,0,1,fid)
        duel::register_flag_effect(d, p, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid);
    }
}
