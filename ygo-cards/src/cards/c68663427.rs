//! Reincarnation Unveiling Mail
use super::prelude::*;

pub const ID: u32 = 68663427;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARMORED_XYZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e0=aux.AddEquipProcedure(c)
    let e0 = aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // Grant the above effect to an Xyz Monster equipped with this card
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e1b:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1b, LOCATION_SZONE);
    // e1b:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1b, LOCATION_MZONE, LOCATION_MZONE);
    // e1b:SetTarget(function(e,c) return e:GetHandler():GetEquipTarget()==c and c:IsType(TYPE_XYZ) end)
    effect::set_target(d, e1b, Some(effect_card_fn(equipped_xyz)));
    // e1b:SetLabelObject(e1)
    effect::set_label_object(d, e1b, Value::Effect(e1));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Return 1 Equip Card equipped to this card to the hand, then, immediately after this effect resolves, Xyz Summon 1 WATER Xyz Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(function(e) return e:GetHandler():GetBattledGroupCount()>0 end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::get_battled_group_count(d, h) > 0
        })),
    );
    // e2:SetTarget(s.xyztg)
    effect::set_target(d, e2, Some(cost_target(xyztg)));
    // e2:SetOperation(s.xyzop)
    effect::set_operation(d, e2, Some(operation(xyzop)));
    // Grant the above effect to an Xyz monster equipped with this card
    // local e2b=e1b:Clone()
    let e2b = effect::clone(d, e1b);
    // e2b:SetLabelObject(e2)
    effect::set_label_object(d, e2b, Value::Effect(e2));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // c:RegisterEffect(e2) is not called in the Lua: e2 is only used as the label object
    // Equip this card to 1 Xyz Monster you control during the End Phase of the turn that it was sent to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e3, CATEGORY_EQUIP);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():GetTurnID()==Duel.GetTurnCount() end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::get_turn_id(d, h) as i64 == duel::get_turn_count(d, None) as i64
        })),
    );
    // e3:SetTarget(s.eqptg)
    effect::set_target(d, e3, Some(cost_target(eqptg)));
    // e3:SetOperation(s.eqpop)
    effect::set_operation(d, e3, Some(operation(eqpop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Any Xyz Monster this card is equipped to becomes an Effect Monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e4, EFFECT_ADD_TYPE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(function(e,c) return e:GetHandler():GetEquipTarget()==c and c:IsType(TYPE_XYZ) end)
    effect::set_target(d, e4, Some(effect_card_fn(equipped_xyz)));
    // e4:SetValue(TYPE_EFFECT)
    effect::set_value(d, e4, TYPE_EFFECT as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e,c) return e:GetHandler():GetEquipTarget()==c and c:IsType(TYPE_XYZ) end`
fn equipped_xyz(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsXyzSummonable()
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_xyz_summonable(d, c, None, None, 0, 0)
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    // local eqpg=e:GetHandler():GetEquipGroup()
    let h = effect::get_handler(d, a.e).nil();
    let eqpg = card::get_equip_group(d, h);
    // if chk==0 then return #eqpg>0 and eqpg:IsExists(Card.IsAbleToHand,1,nil)
    //     and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return !eqpg.is_empty()
            && group::is_exists(d, &eqpg, &|d, c| card::is_able_to_hand(d, c, None), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&xyzfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,eqpg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(eqpg), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=c:GetEquipGroup()
    let g = card::get_equip_group(d, c);
    // if not c:IsRelateToEffect(e) or #g==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local rtc=g:FilterSelect(tp,Card.IsAbleToHand,1,1,nil):GetFirst()
    let sel = group::filter_select(d, &g, a.tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
    let rtc = group::get_first(d, &sel);
    // if not rtc then return end
    let Some(rtc) = rtc else { return };
    // Duel.HintSelection(rtc)
    duel::hint_selection(d, CardSet::from_card(rtc), true);
    // if Duel.SendtoHand(rtc,nil,REASON_EFFECT)>0 and rtc:IsLocation(LOCATION_HAND) then
    if duel::send_to_hand(d, CardSet::from_card(rtc), None, REASON_EFFECT, None) > 0 && card::is_location(d, rtc, LOCATION_HAND) {
        // local sg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil)
        let sg = duel::get_matching_group(d, Some(&xyzfilter), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sc=sg:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.XyzSummon(tp,sc)
            duel::xyz_summon(d, a.tp, sc, None, None, 0, 0);
        }
    }
}
/// `s.eqptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_XYZ))),
                a.tp,
                LOCATION_MZONE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.eqpop(e,tp,eg,ep,ev,re,r,rp)`
fn eqpop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_XYZ))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, CardSet::from_card(sc), true);
        // Duel.Equip(tp,c,sc)
        duel::equip(d, a.tp, c, sc, true, false);
    }
}
