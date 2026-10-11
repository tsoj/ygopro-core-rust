//! Live☆Twin Sunny's Snitch
use super::prelude::*;

pub const ID: u32 = 37582948;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_EVIL_TWIN as u32, SET_LIVE_TWIN as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
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
    // Gain LP & damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIVE_TWIN) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_LIVE_TWIN as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local cg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let cg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #cg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if group::get_count(&cg) > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=cg:Select(tp,1,1,nil)
        let sg = group::select(d, &cg, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) then return false end
    let eg = a.eg.clone().nil();
    if !group::is_exists(d, &eg, &|d, x| card::is_summon_player(d, x, 1 - a.tp), 1, NO_EX) {
        return false;
    }
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_EVIL_TWIN),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_EVIL_TWIN as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // if Duel.Recover(tp,200,REASON_EFFECT)>0 then
    if duel::recover(d, a.tp, 200, REASON_EFFECT, false, None) > 0 {
        // Duel.Damage(1-tp,200,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 200, REASON_EFFECT, false, None);
    }
}
