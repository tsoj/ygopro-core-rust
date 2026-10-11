//! Labyrinth Heavy Tank
//! Scripted by Satella
use super::prelude::*;

pub const ID: u32 = 35026117;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: CARDS_SANGA_KAZEJIN_SUIJIN, listed_series: &[SET_LABYRINTH_WALL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot attack the turn it is Normal Summoned
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetOperation(s.disatt)
    effect::set_operation(d, e2, Some(operation(disatt)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 1 Sanga, Kazejin, or Suijin face-up in your S&T Zone as a Continuous Spell
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.placetg)
    effect::set_target(d, e3, Some(cost_target(placetg)));
    // e3:SetOperation(s.placeop)
    effect::set_operation(d, e3, Some(operation(placeop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0
        && card::get_level(d, c) > 4
        && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.disatt(e,tp,eg,ep,ev,re,r,rp)`
fn disatt(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARDS_SANGA_KAZEJIN_SUIJIN) and not c:IsForbidden() and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED))
    card::is_code(d, c, CARDS_SANGA_KAZEJIN_SUIJIN)
        && !card::is_forbidden(d, c)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
}
/// `s.placetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn placetg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_REMOVED|LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_REMOVED | LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.placeop(e,tp,eg,ep,ev,re,r,rp)`
fn placeop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_REMOVED|LOCATION_HAND|LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&plfilter), a.tp, LOCATION_REMOVED | LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if let Some(tc) = tc {
        if duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // Treated as a Continuous Spell
            // local e1=Effect.CreateEffect(e:GetHandler())
            let handler = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, handler);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_LABYRINTH_WALL),tp,LOCATION_ONFIELD,0,1,nil)
            //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_LABYRINTH_WALL as u16])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
                && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
                && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
                // local sg=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
                let sg = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(sg,true)
                duel::hint_selection(d, sg.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(sg,REASON_EFFECT)
                duel::destroy(d, sg, REASON_EFFECT, None, None);
            }
        }
    }
}
