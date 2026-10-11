//! Krishnerd Witch
use super::prelude::*;

pub const ID: u32 = 37006702;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[885016], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by opponent's card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(indes_cond)));
    effect::set_value_fn(d, e1, aux::indoval());
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Shuffle 1 card to the Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(tdcon)));
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    effect::set_operation(d, e3, Some(operation(tdop)));
    card::register_effect(d, c, e3, false);
}
fn indes_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,885016),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[885016]))),
        0,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        NO_EX,
    )
}
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_FZONE) and c:IsReason(REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_FZONE) && card::is_reason(d, c, REASON_EFFECT)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &spconfilter, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler():IsOnField() and re:IsActiveType(TYPE_FIELD) and not re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    card::is_on_field(d, h) && effect::is_active_type(d, re, TYPE_FIELD) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `Duel.GetChainInfo(ev, CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2)`; nil values are `None`.
fn chain_codes(d: &mut Duel, ev: u32) -> (Option<u32>, Option<u32>) {
    let v = duel::get_chain_info(d, ev as u8, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
    (to_code(&v[0]), to_code(&v[1]))
}
fn to_code(v: &Value) -> Option<u32> {
    match v {
        Value::Nil => None,
        v => Some(v.int() as u32),
    }
}
/// The codes `c:ListsCode(code1, code2)` gets: Lua's `{...}` stops at the first nil.
fn codes(code1: Option<u32>, code2: Option<u32>) -> Vec<u32> {
    let mut v = Vec::new();
    for x in [code1, code2] {
        match x {
            Some(x) => v.push(x),
            None => break,
        }
    }
    v
}
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32, code1: Option<u32>, code2: Option<u32>) -> bool {
    // return c:IsMonster() and c:IsFaceup() and (c:IsAbleToDeck() or (ft>0 and s.spfilter(c,e,tp,code1,code2)))
    card::is_monster(d, c)
        && card::is_faceup(d, c)
        && (card::is_able_to_deck(d, c) || (ft > 0 && spfilter(d, c, e, tp, code1, code2)))
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code1: Option<u32>, code2: Option<u32>) -> bool {
    // return c:ListsCode(code1,code2) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::lists_code(d, c, &codes(code1, code2)) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // local code1,code2=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CODE,CHAININFO_TRIGGERING_CODE2)
        let (code1, code2) = chain_codes(d, a.ev);
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| tdfilter(d, c, a.e, a.tp, ft, code1, code2)),
            a.tp,
            LOCATION_REMOVED | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_REMOVED | LOCATION_GRAVE) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_REMOVED | LOCATION_GRAVE) as i32);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local code1,code2=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CODE,CHAININFO_TRIGGERING_CODE2)
    let (code1, code2) = chain_codes(d, a.ev);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_REMOVED|LOCATION_GRAVE,0,1,1,nil,e,tp,ft,code1,code2):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| tdfilter(d, c, a.e, tp, ft, code1, code2))),
        tp,
        LOCATION_REMOVED | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // local b1=tc:IsAbleToDeck()
    let b1 = card::is_able_to_deck(d, tc);
    // local b2=ft>0 and s.spfilter(tc,e,tp,code1,code2)
    let b2 = ft > 0 && spfilter(d, tc, a.e, tp, code1, code2);
    // local op=nil
    // if b1 and b2 then
    //   op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    // else
    //   op=(b1 and 1) or (b2 and 2)
    // end
    let op = if b1 && b2 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
    } else if b1 {
        Some(1)
    } else if b2 {
        Some(2)
    } else {
        None
    };
    if op == Some(1) {
        // Duel.HintSelection(tc)
        duel::hint_selection(d, tc, true);
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    } else if op == Some(2) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
