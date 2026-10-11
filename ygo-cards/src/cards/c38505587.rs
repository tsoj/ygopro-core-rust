//! Moremarshmallon
use super::prelude::*;

pub const ID: u32 = 38505587;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SHINING_SARCOPHAGUS, ID], ..ScriptInfo::DEFAULT };

/// `LOCATIONS_REMOVED_HAND_DECK_GRAVE`
const LOCATIONS_REMOVED_HAND_DECK_GRAVE: u32 = LOCATION_REMOVED | LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.goldsarccon)
    effect::set_condition(d, e1, Some(condition(goldsarccon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent's monsters cannot target other monsters for attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.goldsarccon)
    effect::set_condition(d, e2, Some(condition(goldsarccon)));
    // e2:SetValue(function(e,_c) return _c~=e:GetHandler() end)
    effect::set_value_fn(d, e2, lua_fn(not_self_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card from your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetHintTiming(0,TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and s.goldsarccon(e) end)
    effect::set_condition(d, e3, Some(condition(selfsp_cond)));
    // e3:SetTarget(s.selfsptg)
    effect::set_target(d, e3, Some(cost_target(selfsptg)));
    // e3:SetOperation(s.selfspop)
    effect::set_operation(d, e3, Some(operation(selfspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 other "Moremarshmallon" and inflict 1000 damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(function(e) return e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e4, Some(condition(destroyed_by_effect_cond)));
    // e4:SetTarget(s.marshsptg)
    effect::set_target(d, e4, Some(cost_target(marshsptg)));
    // e4:SetOperation(s.marshspop)
    effect::set_operation(d, e4, Some(operation(marshspop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e,_c) return _c~=e:GetHandler() end`
fn not_self_value(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0));
    a.card(1) != h
}
/// `s.goldsarccon(e)`
fn goldsarccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SHINING_SARCOPHAGUS),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS])),
        p,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) and s.goldsarccon(e) end`
fn selfsp_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && goldsarccon(d, a)
}
/// `function(e) return e:GetHandler():IsReason(REASON_EFFECT) end`
fn destroyed_by_effect_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_EFFECT)
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.marshspfilter(c,e,tp)`
fn marshspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(id) and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[ID])
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.marshsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn marshsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.marshspfilter,tp,LOCATIONS_REMOVED_HAND_DECK_GRAVE,0,1,e:GetHandler(),e,tp) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| marshspfilter(d, x, e, tp)),
                tp,
                LOCATIONS_REMOVED_HAND_DECK_GRAVE,
                0,
                1,
                c.into(),
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATIONS_REMOVED_HAND_DECK_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATIONS_REMOVED_HAND_DECK_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
    true
}
/// `s.marshspop(e,tp,eg,ep,ev,re,r,rp)`
fn marshspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc: Option<CardId> = if card::is_relate_to_effect(d, c, e) { Some(c) } else { None };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.marshspfilter),tp,LOCATIONS_REMOVED_HAND_DECK_GRAVE,0,1,1,exc,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| marshspfilter(d, x, e, tp))),
        tp,
        LOCATIONS_REMOVED_HAND_DECK_GRAVE,
        0,
        1,
        1,
        false,
        exc.into(),
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Damage(1-tp,1000,REASON_EFFECT)
        duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
    }
}
