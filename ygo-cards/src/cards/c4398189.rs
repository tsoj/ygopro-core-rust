//! Witch of the White Forest
use super::prelude::*;

pub const ID: u32 = 4398189;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WHITE_FOREST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // The first time each "White Forest" monster you control would be destroyed by battle each turn, it is not destroyed
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_WHITE_FOREST))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_WHITE_FOREST as u16]))));
    // e2:SetValue(function(e,re,r,rp) return r&REASON_BATTLE==REASON_BATTLE end)
    effect::set_value_fn(d, e2, lua_fn(value_fn));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make 1 "White Forest" monster you control become a Tuner that turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.tunertg)
    effect::set_target(d, e3, Some(cost_target(tunertg)));
    // e3:SetOperation(s.tunerop)
    effect::set_operation(d, e3, Some(operation(tunerop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WHITE_FOREST) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_WHITE_FOREST as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if Duel.SendtoHand(sg,nil,REASON_EFFECT)>0 and sg:GetFirst():IsLocation(LOCATION_HAND) then
        if duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) > 0 {
            let first = group::get_first(d, &sg).nil();
            if card::is_location(d, first, LOCATION_HAND) {
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - tp, sg);
                // local c=e:GetHandler()
                let c = effect::get_handler(d, a.e).nil();
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(aux.Stringid(id,3))
                effect::set_description(d, e1, aux::stringid(ID, 3));
                // e1:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD);
                // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
                // e1:SetTargetRange(1,0)
                effect::set_target_range(d, e1, 1, 0);
                // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and c:IsAttribute(ATTRIBUTE_DARK) end)
                effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && card::is_attribute(d, c, ATTRIBUTE_DARK))));
                // e1:SetReset(RESET_PHASE|PHASE_END)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, tp);
                // aux.addTempLizardCheck(c,tp,function(e,c) return c:IsOriginalAttribute(ATTRIBUTE_DARK) end)
                aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| card::is_original_attribute(d, c, ATTRIBUTE_DARK))), None, None, None, None);
            }
        }
    }
}
// function s.nontunertgfilter(c)
fn nontunertgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WHITE_FOREST) and not c:IsType(TYPE_TUNER) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_WHITE_FOREST as u16]) && !card::is_type(d, c, TYPE_TUNER) && card::is_faceup(d, c)
}
// function s.tunertg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tunertg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.nontunertgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && nontunertgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.nontunertgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&nontunertgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.nontunertgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&nontunertgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.tunerop(e,tp,eg,ep,ev,re,r,rp)
fn tunerop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `function(e,re,r,rp) return r&REASON_BATTLE==REASON_BATTLE end`
fn value_fn(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) & REASON_BATTLE as i64) == REASON_BATTLE as i64
}
