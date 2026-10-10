//! Rilliona, the Magistus of Verre
use super::prelude::*;

pub const ID: u32 = 72498838;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.efftg)
    effect::set_target(d, e1a, Some(cost_target(efftg)));
    // e1a:SetOperation(s.effop)
    effect::set_operation(d, e1a, Some(operation(effop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Equip 1 "Magistus" monster you control with 1 "Magistus" monster from your GY, except a Level 4 monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGISTUS) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MAGISTUS as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.rtgfilter(c)`
fn rtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_SPELLCASTER) and c:IsFaceup()
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_faceup(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.rtgfilter,tp,LOCATION_REMOVED,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&rtgfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Add 1 "Magistus" Spell/Trap from your Deck to your hand
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
    // elseif op==2 then
    } else if op == 2 {
        // Return 1 of your banished Level 4 or lower Spellcaster monsters to your GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.rtgfilter,tp,LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&rtgfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoGrave(g,REASON_EFFECT|REASON_RETURN)
            duel::send_to_grave(d, g, REASON_EFFECT | REASON_RETURN, None, None);
        }
    }
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGISTUS) and not c:IsLevel(4) and c:IsMonster()
    card::is_set_card(d, c, &[SET_MAGISTUS as u16]) && !card::is_level(d, c, &[4]) && card::is_monster(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsSetCard(SET_MAGISTUS) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && card::is_set_card(d, chkc, &[SET_MAGISTUS as u16])
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_MAGISTUS),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MAGISTUS as u16]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&eqfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_MAGISTUS),tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MAGISTUS as u16]))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&eqfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &sel);
        // if ec and Duel.Equip(tp,ec,tc) then
        if let Some(ec) = ec {
            if duel::equip(d, tp, ec, tc, true, false) {
                // Equip limit
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                // e1:SetValue(function(e,c) return c==tc end)
                effect::set_value_fn(d, e1, effect_card_fn(move |_d, _e, x| x == tc));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // ec:RegisterEffect(e1)
                card::register_effect(d, ec, e1, false);
            }
        }
    }
}
