//! Call of the Forgotten
use super::prelude::*;

pub const ID: u32 = 80749819;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CALL_OF_THE_HAUNTED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can Set up to 3 "Call of the Haunted" from your Deck and/or GY, and if you do, until the end of your opponent's turn, you cannot Special Summon, except Zombie monsters, or monsters from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
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
    // Once per turn, if you activate "Call of the Haunted": You can target 1 monster your opponent controls; send it to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.gycon)
    effect::set_condition(d, e2, Some(condition(gycon)));
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_CALL_OF_THE_HAUNTED) and c:IsSSetable()
    card::is_code(d, c, &[CARD_CALL_OF_THE_HAUNTED]) && card::is_s_setable(d, c, false, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| setfilter(d, c))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_SZONE),3)
        let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0.min(3) as u16;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        // local sg=g:Select(tp,1,ft,nil)
        let sg = group::select(d, &g, a.tp, 1, ft, false, NO_EX).nil();
        // if #sg>0 and Duel.SSet(tp,sg)>0 then
        if !sg.is_empty() && duel::s_set(d, a.tp, sg, None, true) > 0 {
            // --Until the end of your opponent's turn, you cannot Special Summon, except Zombie monsters, or monsters from the GY
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,3))
            effect::set_description(d, e1, aux::stringid(ID, 3));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetTarget(function(e,c) return c:IsRaceExcept(RACE_ZOMBIE) and not c:IsLocation(LOCATION_GRAVE) end)
            effect::set_target(
                d,
                e1,
                Some(effect_card_fn(|d, _e, c| {
                    card::is_race_except(d, c, RACE_ZOMBIE as u64, None, 0, None) && !card::is_location(d, c, LOCATION_GRAVE)
                })),
            );
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
/// `s.gycon(e,tp,eg,ep,ev,re,r,rp)`
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(CARD_CALL_OF_THE_HAUNTED)
    if a.rp != a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    card::is_code(d, rc, &[CARD_CALL_OF_THE_HAUNTED])
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_grave(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, a.tp, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoGrave(tc,REASON_EFFECT)
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
