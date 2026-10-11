//! Elemental HERO Nebula Neos
use super::prelude::*;

pub const ID: u32 = 40080312;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS, 80344569, 43237273],
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO, SET_NEOS, SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Elemental HERO Neos" + "Neo-Spacian Grand Mole" + "Neo-Spacian Dark Panther"
    // Fusion.AddProcMix(c,true,true,CARD_NEOS,80344569,43237273)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_NEOS), fusion::Mat::Code(80344569), fusion::Mat::Code(43237273)]);
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
    // aux.EnableNeosReturn(c,CATEGORY_REMOVE,s.retinfo,s.retop)
    aux::enable_neos_return(d, c, Some(CATEGORY_REMOVE), Some(operation(retinfo)), Some(operation(retop)), None);
    // Draw cards equal to the number of cards your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_EXTRA) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_EXTRA)
    })));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck_or_extra_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.contactop(g,tp)`
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA)
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, h, LOCATION_EXTRA)
}
/// `s.retinfo(e,tp,eg,ep,ev,re,r,rp)`
fn retinfo(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler(),tp,POS_FACEDOWN)
    // (the `1` in the exception position is a number: no exception; the filter gets (c, 1, handler, tp, POS_FACEDOWN),
    // so IsAbleToRemove reads player 1, the default position (a card is no number) and reason tp)
    let _h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(1), Some(POS_FACEUP as u8), Some(a.tp as u32))), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, a.tp, 0);
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil,tp,POS_FACEDOWN)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(1), Some(POS_FACEUP as u8), Some(a.tp as u32))), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local d=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let dcount = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if Duel.Draw(p,d,REASON_EFFECT)>0 and #g>0 then
    if duel::draw(d, p, dcount as u32, REASON_EFFECT) > 0 && !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // tc:NegateEffects(e:GetHandler(),RESETS_STANDARD_PHASE_END,true)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESETS_STANDARD_PHASE_END), true, None);
    }
}
