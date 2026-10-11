//! Triamid Dancer
use super::prelude::*;

pub const ID: u32 = 69529337;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRIAMID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle 1 "Triamid" card into the Deck and increase the ATK/DEF of Rock monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 "Triamid" Field Spell from the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(turncond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end` (the second effect's condition)
fn turncond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRIAMID) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_TRIAMID as u16]) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_ROCK),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_ROCK)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0
        && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
    {
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_ROCK),tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_ROCK)), a.tp, LOCATION_MZONE, 0, NO_EX);
        // for sc in g:Iter() do
        for sc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(500)
            effect::set_value(d, e1, 500);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
        }
    }
}
/// `s.filter(c,tp,code)`
fn filter(d: &mut Duel, c: CardId, tp: u8, code: u32) -> bool {
    // return c:IsFieldSpell() and c:IsSetCard(SET_TRIAMID) and c:GetActivateEffect():IsActivatable(tp,true, true) and not c:IsCode(code)
    let act = card::get_activate_effect(d, c).first().copied().nil();
    card::is_field_spell(d, c) && card::is_set_card(d, c, &[SET_TRIAMID as u16]) && effect::is_activatable(d, act, tp, true, true) && !card::is_code(d, c, &[code])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let tc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return tc and tc:IsFaceup() and tc:IsSetCard(SET_TRIAMID) and tc:IsAbleToGrave() and tc:IsCanBeEffectTarget(e)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,tp,tc:GetCode()) end
    if a.chk == Some(0) {
        return match tc {
            None => false,
            Some(tc) => {
                card::is_faceup(d, tc)
                    && card::is_set_card(d, tc, &[SET_TRIAMID as u16])
                    && card::is_able_to_grave(d, tc, None)
                    && card::is_can_be_effect_target(d, tc, Some(a.e))
                    && {
                        let code = card::get_code(d, tc)[0];
                        duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, a.tp, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
                    }
            }
        };
    }
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(tc.nil())), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoGrave(tc,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_GRAVE) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_grave(d, tc, REASON_EFFECT, None, None) != 0 && card::is_location(d, tc, LOCATION_GRAVE) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local fc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,tp,tc:GetCode()):GetFirst()
        let code = card::get_code(d, tc)[0];
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, a.tp, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let fc = group::get_first(d, &sel);
        // Duel.ActivateFieldSpell(fc,e,tp,eg,ep,ev,re,r,rp)
        duel::activate_field_spell(d, fc, a, None);
    }
}
