//! Number 77: The Seven Sins
use super::prelude::*;

pub const ID: u32 = 62541668;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(77), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,12,2,s.ovfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(12),
        2,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Banish as many Special Summoned monsters then attach 1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Protect itself from destruction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_DARK,xyzc,SUMMON_TYPE_XYZ,tp)
    //     and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and (rk==10 or rk==11)
    card::is_faceup(d, c)
        && card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && (rk == 10 || rk == 11)
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    // if chk==0 then return true end
    if a.int(2) == 0 {
        return true;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD),0,1)
    let c = effect::get_handler(d, a.e(0)).nil();
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 0, 1, 0, 0);
    // return true
    true
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned() and c:IsAbleToRemove() and c:IsMonster()
    card::is_special_summoned(d, c) && card::is_able_to_remove(d, c, None, None, None) && card::is_monster(d, c)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::get_flag_effect(d, c, ID) == 0 && duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = group::get_count(&g);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n as u8, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if group::get_count(&g) == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)==0 or not c:IsRelateToEffect(e) then return end
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_REMOVED)
    let operated = duel::get_operated_group(d);
    let og = group::filter(d, &operated, &|d, x| card::is_location(d, x, LOCATION_REMOVED), NO_EX);
    // if #og>0 then
    if group::get_count(&og) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // local sg=og:Select(tp,1,1,nil)
        let sg = group::select(d, &og, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Overlay(c,sg)
        duel::overlay(d, c, sg, false);
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
            && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
        // return true
        true
    // else return false end
    } else {
        false
    }
}
