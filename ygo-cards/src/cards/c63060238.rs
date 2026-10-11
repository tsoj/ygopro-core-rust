//! Elemental HERO Blazeman
use super::prelude::*;

pub const ID: u32 = 63060238;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO],
    listed_names: &[CARD_POLYMERIZATION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 "Polymerization"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 "Elemental HERO" monster from your Deck to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and not c:IsCode(id) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return not c:IsType(TYPE_FUSION)
        !card::is_type(d, c, TYPE_FUSION)
    } else {
        // return not c:IsOriginalType(TYPE_FUSION)
        !card::is_original_type(d, c, TYPE_FUSION)
    }
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE)
    //     and c:IsFaceup() and c:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0
            && card::is_location(d, tc, LOCATION_GRAVE)
            && card::is_faceup(d, c)
            && card::is_relate_to_effect(d, c, a.e)
        {
            // Change this card's Attribute and ATK/DEF
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
            // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
            effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
            // e1:SetValue(tc:GetAttribute())
            let attr = card::get_attribute(d, tc);
            effect::set_value(d, e1, attr as i64);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
            // e2:SetValue(tc:GetAttack())
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e2, atk as i64);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // local e3=e1:Clone()
            let e3 = effect::clone(d, e1);
            // e3:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e3, EFFECT_SET_DEFENSE_FINAL);
            // e3:SetValue(tc:GetDefense())
            let def = card::get_defense(d, tc);
            effect::set_value(d, e3, def as i64);
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
        }
    }
    // Cannot Special Summon, except Fusion Monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetTarget(s.splimit)
    effect::set_target(d, e4, Some(lua_fn(splimit)));
    // e4:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, tp);
    // Cannot Special Summon from the Main Deck check
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetCode(CARD_EHERO_BLAZEMAN)
    effect::set_code(d, e5, CARD_EHERO_BLAZEMAN);
    // e5:SetTargetRange(1,0)
    effect::set_target_range(d, e5, 1, 0);
    // e5:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e5, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e5,tp)
    duel::register_effect(d, e5, tp);
}
