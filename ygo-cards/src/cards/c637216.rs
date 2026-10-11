//! Monster Express
use super::prelude::*;

pub const ID: u32 = 637216;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 monster from extra deck to GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter2(c,race)`
fn filter2(d: &mut Duel, c: CardId, race: u64) -> bool {
    // return c:IsAbleToGrave() and c:GetOriginalRace()==race
    card::is_able_to_grave(d, c, None) && card::get_original_race(d, c) == race
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local race=c:GetOriginalRace()
    let race = card::get_original_race(d, c);
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,race)
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, race)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter1(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter1(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let _g = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,tc:GetOriginalRace())
            let race = card::get_original_race(d, tc);
            let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter2(d, c, race)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_GRAVE) then
            if !g.is_empty() && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) > 0 {
                let first = group::get_first(d, &g).nil();
                if card::is_location(d, first, LOCATION_GRAVE) {
                    // local e1=Effect.CreateEffect(e:GetHandler())
                    let h = effect::get_handler(d, a.e).nil();
                    let e1 = effect::create_effect(d, h);
                    // e1:SetDescription(aux.Stringid(id,1))
                    effect::set_description(d, e1, aux::stringid(ID, 1));
                    // e1:SetType(EFFECT_TYPE_FIELD)
                    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
                    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
                    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
                    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
                    // e1:SetTargetRange(1,0)
                    effect::set_target_range(d, e1, 1, 0);
                    // e1:SetTarget(s.splimit(g:GetFirst()))
                    let limited = first;
                    effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, c| {
                        // return c:GetOriginalRace()~=tc:GetOriginalRace()
                        card::get_original_race(d, c) != card::get_original_race(d, limited)
                    })));
                    // e1:SetReset(RESET_PHASE|PHASE_END)
                    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
                    // Duel.RegisterEffect(e1,tp)
                    duel::register_effect(d, e1, a.tp);
                }
            }
        }
    }
}
