//! A-to-Z-Dragon Buster Cannon
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 65172015;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[1561110, 91998119], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // if not c:IsStatus(STATUS_COPYING_EFFECT) then
    if !card::is_status(d, c, STATUS_COPYING_EFFECT) {
        // Fusion.AddContactProc(c,s.contactfil,s.contactop,aux.FALSE)
        fusion::add_contact_proc(
            d,
            c,
            fusion::Contact {
                material_filter: Some(Arc::new(contactfil)),
                material_operation: Some(Arc::new(contactop)),
                sumcon: Some(Some(aux_false())),
                ..Default::default()
            },
        );
        // if c.material_count==nil then
        if crate::class::get(d, c, crate::class::MATERIAL_COUNT).is_nil() {
            // s.material_count=2
            crate::class::set(d, c, crate::class::MATERIAL_COUNT, Value::Int(2));
            // s.material={1561110,91998119}
            crate::class::set(d, c, crate::class::MATERIAL, Value::Multi(vec![Value::Int(1561110), Value::Int(91998119)]));
            // s.min_material_count=2
            crate::class::set(d, c, crate::class::MIN_MATERIAL_COUNT, Value::Int(2));
            // s.max_material_count=2
            crate::class::set(d, c, crate::class::MAX_MATERIAL_COUNT, Value::Int(2));
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetCode(EFFECT_FUSION_MATERIAL)
        effect::set_code(d, e1, EFFECT_FUSION_MATERIAL);
        // e1:SetCondition(Fusion.ConditionMix(true,true,s.ffilter(1561110),s.ffilter(91998119)))
        let funs: Arc<[fusion::Fun]> = Arc::from(vec![ffilter(1561110), ffilter(91998119)]);
        effect::set_condition(d, e1, Some(fusion::condition_mix(true, true, funs.clone())));
        // e1:SetOperation(Fusion.OperationMix(true,true,s.ffilter(1561110),s.ffilter(91998119)))
        effect::set_operation(d, e1, Some(fusion::operation_mix(true, funs)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // Negate activations
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon banished monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop2)
    effect::set_operation(d, e3, Some(operation(spop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.ffilter(code)`: `function(c,fc,sub,sub2,mg,sg,tp,contact)`
fn ffilter(code: u32) -> fusion::Fun {
    Arc::new(move |d, c, x| {
        if x.contact {
            card::is_original_code(d, c, &[code])
        } else {
            card::is_summon_code(d, c, Some(x.fc), SUMMON_TYPE_FUSION as u64, x.tp, &[code])
                || (x.sub && card::check_fusion_substitute(d, c, x.fc))
                || (x.sub2 && !card::is_has_effect(d, c, 511002961, None).is_empty())
        }
    })
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    a.ep == 1 - a.tp
        && !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.discard_filter`: `Card.IsDiscardable`
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discardable), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let h = effect::get_handler(d, a.re.nil()).nil();
    if card::is_relate_to_effect(d, h, a.re.nil()) && card::is_destructable(d, h, None) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let h = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, h, a.re.nil()) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[1561110]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_target(d, Some(&|d, x| spfilter3(d, x, e, tp)), tp, LOCATION_REMOVED, 0, 1, c.into())
}
/// `s.spfilter3(c,e,tp)`
fn spfilter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[91998119]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        let h = effect::get_handler(d, a.e).nil();
        if card::get_sequence(d, h) < 5 {
            ft += 1;
        }
        return ft > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_target(d, Some(&|d, x| spfilter2(d, x, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectTarget(tp,s.spfilter2,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let mut g1 = duel::select_target(d, a.tp, Some(&|d, x| spfilter2(d, x, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(tp,s.spfilter3,tp,LOCATION_REMOVED,0,1,1,g1:GetFirst(),e,tp)
    let first = group::get_first(d, &g1);
    let ex = first.map_or(NO_EX, |x| x.into());
    let g2 = duel::select_target(d, a.tp, Some(&|d, x| spfilter3(d, x, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, ex).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g1,2,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), 2, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if g.is_empty() || (g.len() > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    // if #g<=ft then
    if g.len() as i32 <= ft {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,ft,ft,nil)
        let sg = group::select(d, &g, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // g:Sub(sg)
        group::remove_card(&mut g, &sg);
        // Duel.SendtoGrave(g,REASON_RULE)
        duel::send_to_grave(d, g, REASON_RULE, None, None);
    }
}
