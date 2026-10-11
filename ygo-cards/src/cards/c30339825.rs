//! Hydor, the Base of All Things
//! --万物の始源－「水」
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 30339825;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster from either GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg(false))
    effect::set_target(d, e1, Some(cost_target(|d, a| sptg(d, a, false))));
    // e1:SetOperation(s.spop(false))
    effect::set_operation(d, e1, Some(operation(|d, a| spop(d, a, false))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 WATER monster from either GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg(true))
    effect::set_target(d, e2, Some(cost_target(|d, a| sptg(d, a, true))));
    // e2:SetOperation(s.spop(true))
    effect::set_operation(d, e2, Some(operation(|d, a| spop(d, a, true))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, water: bool) -> bool {
    // return (not water or c:IsAttribute(ATTRIBUTE_WATER)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //     and Duel.IsExistingMatchingCard(s.desfilter,c:GetOwner(),LOCATION_MZONE,0,1,nil,water)
    (!water || card::is_attribute(d, c, ATTRIBUTE_WATER))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && {
            let owner = card::get_owner(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, water)), owner, LOCATION_MZONE, 0, 1, NO_EX)
        }
}
fn desfilter(d: &mut Duel, c: CardId, water: bool) -> bool {
    // return Duel.GetMZoneCount(c:GetControler(),c)>0 and (water or (c:IsAttribute(ATTRIBUTE_WATER) and c:IsFaceup()))
    let p = card::get_controler(d, c);
    duel::get_m_zone_count(d, p, c.into(), None, None, None).0 > 0
        && (water || (card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_faceup(d, c)))
}
fn sptg(d: &mut Duel, a: &EffArgs, water: bool) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,water) end
    // The Lua passes water as the tp argument here and nil as water: IsCanBeSpecialSummoned then
    // gets a boolean where it reads a player, and the core raises an error.
    if let Some(chkc) = a.chkc() {
        if card::is_location(d, chkc, LOCATION_GRAVE) {
            crate::lua_error("Parameter 4 should be \"INT\" but is \"boolean\".");
        }
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp,water) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp, water)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp,water)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp, water)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs, water: bool) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.desfilter,tc:GetOwner(),LOCATION_MZONE,0,1,1,nil,water)
    let owner = card::get_owner(d, tc);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| desfilter(d, c, water)),
        owner,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    );
    // if #g==0 then return end
    let Some(g) = g else { return };
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.Destroy(g,REASON_EFFECT)>0 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // Duel.SpecialSummon(tc,0,tp,tc:GetControler(),false,false,POS_FACEUP_DEFENSE)
        let p = card::get_controler(d, tc);
        duel::special_summon(d, tc, 0, tp, p, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
