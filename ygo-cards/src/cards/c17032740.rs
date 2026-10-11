//! Elemental HERO Chaos Neos
use super::prelude::*;

pub const ID: u32 = 17032740;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO, SET_NEOS, SET_NEO_SPACIAN],
    toss_coin: true,
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Summon Procedure
    // Fusion.AddProcMix(c,true,true,CARD_NEOS,43237273,17732278)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_NEOS), fusion::Mat::Code(43237273), fusion::Mat::Code(17732278)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // aux.EnableNeosReturn(c,CATEGORY_POSITION+CATEGORY_SET,s.retinfo,s.retop)
    aux::enable_neos_return(d, c, Some(CATEGORY_POSITION | CATEGORY_SET), Some(lua_fn(retinfo)), Some(lua_fn(retop)), None);
    // Toss a coin 3 times and apply the appropriate effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_COIN+CATEGORY_DESTROY+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_COIN | CATEGORY_DESTROY | CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.coincon)
    effect::set_condition(d, e1, Some(condition(coincon)));
    // e1:SetTarget(s.cointg)
    effect::set_target(d, e1, Some(cost_target(cointg)));
    // e1:SetOperation(s.coinop)
    effect::set_operation(d, e1, Some(operation(coinop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck_or_extra_as_cost(d, x)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.contactop(g,tp)`
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA)
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, h, LOCATION_EXTRA)
}
/// `s.retinfo(e,tp,eg,ep,ev,re,r,rp)`
fn retinfo(d: &mut Duel, _a: Args) {
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_ALL,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_ALL as u8, POS_FACEDOWN_DEFENSE as i32);
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
    duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
}
/// `s.coincon(e,tp,eg,ep,ev,re,r,rp)`
fn coincon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    duel::is_phase(d, PHASE_MAIN1)
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 3);
    true
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local total_heads=Duel.CountHeads(Duel.TossCoin(tp,3))
    let results = duel::toss_coin(d, tp, 3);
    let total_heads = duel::count_heads(&results);
    // if total_heads==3 then
    if total_heads == 3 {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
        let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else if total_heads == 2 {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, NO_EX);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    } else if total_heads == 1 {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), tp, LOCATION_MZONE, 0, NO_EX);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
