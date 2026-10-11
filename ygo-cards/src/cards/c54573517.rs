//! T.G. All Clear
use super::prelude::*;

pub const ID: u32 = 54573517;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TG],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "T.G." monsters on the field become Machine monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e2, EFFECT_CHANGE_RACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_TG))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TG as u16]))));
    // e2:SetValue(RACE_MACHINE)
    effect::set_value(d, e2, RACE_MACHINE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Additional "T.G." Normal Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e3, EFFECT_EXTRA_SUMMON_COUNT);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_HAND | LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_TG))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TG as u16]))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy 1 "T.G." monster and add 1 "T.G." monster with a different name to your hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.desfilter(c,tp)
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TG) and c:IsMonster() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,c:GetCode())
    let code = card::get_code(d, c)[0];
    card::is_set_card(d, c, &[SET_TG as u16])
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, code)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.thfilter(c,code)
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsSetCard(SET_TG) and c:IsMonster() and not c:IsCode(code) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TG as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[code])
        && card::is_able_to_hand(d, c, None)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_HAND, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &sel) else { return };
    // local code=tc:GetCode()
    let code = card::get_code(d, tc)[0];
    // if Duel.Destroy(tc,REASON_EFFECT)>0 then
    if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local thg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,code)
        let thg = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(move |d, c| thfilter(d, c, code))),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #thg>0 then
        if group::get_count(&thg) > 0 {
            // Duel.SendtoHand(thg,nil,REASON_EFFECT)
            duel::send_to_hand(d, thg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,thg)
            duel::confirm_cards(d, 1 - tp, thg);
        }
    }
}
