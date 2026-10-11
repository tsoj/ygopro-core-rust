//! Luster Pendulum, the Dracoslayer
use super::prelude::*;

pub const ID: u32 = 92746535;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOSLAYER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Destroy a card in your Pendulum Zone and add 1 card with the same name from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be Fusion, Synchro or Xyz material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e2:SetValue(s.splimit)
    effect::set_value_fn(d, e2, lua_fn(splimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(nil,tp,LOCATION_PZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, None, a.tp, LOCATION_PZONE, 0, 1, h.into())
}
/// `s.thfilter(c,code)`
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsAbleToHand()
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local sc=Duel.GetFirstMatchingCard(nil,tp,LOCATION_PZONE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let sc = duel::get_first_matching_card(d, None, a.tp, LOCATION_PZONE, 0, h.into()).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,sc:GetCode()) end
    if a.chk == Some(0) {
        let code = card::get_code(d, sc).first().copied().unwrap_or(0);
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetTargetCard(sc)
    duel::set_target_card(d, sc);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(sc)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetCode())
        let code = card::get_code(d, tc).first().copied().unwrap_or(0);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, &g);
        }
    }
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // if not c then return false end
    let c = match a.card(1) {
        Some(c) => c,
        None => return false,
    };
    // return not c:IsSetCard(SET_DRACOSLAYER)
    !card::is_set_card(d, c, &[SET_DRACOSLAYER as u16])
}
