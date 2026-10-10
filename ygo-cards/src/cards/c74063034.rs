//! Invocation
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[86120751], listed_series: &[SET_INVOKED], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 74063034;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Fusion Monster from your Extra Deck, using monsters from your hand as Fusion Material. If Summoning an "Invoked" Fusion Monster this way, you can also banish monsters from your field and/or either player's GY as Fusion Material
    // local e1=Fusion.CreateSummonEff(c,nil,s.matfilter,s.fextra,s.extraop,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| matfilter(d, c))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::extra_op(extraop)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is in your GY: You can target 1 of your banished "Aleister the Invoker"; shuffle this card into the Deck, and if you do, add that monster to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) and c:IsAbleToGrave()) or (c:IsOnField() and c:IsAbleToRemove())
    (card::is_location(d, c, LOCATION_HAND) && card::is_able_to_grave(d, c, None)) || (card::is_on_field(d, c) && card::is_able_to_remove(d, c, None, None, None))
}
/// `s.checkmat(tp,sg,fc)`
fn checkmat(d: &mut Duel, a: Args) -> bool {
    // return fc:IsSetCard(SET_INVOKED) or not sg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE|LOCATION_ONFIELD)
    let sg = a.g(1);
    let fc = a.c(2);
    card::is_set_card(d, fc, &[SET_INVOKED as u16]) || !group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE | LOCATION_ONFIELD), 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil),s.checkmat
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_GRAVE,
            LOCATION_GRAVE,
            NO_EX,
        );
        return (Some(g), Some(lua_fn(checkmat)));
    }
    // return nil,s.checkmat
    (None, Some(lua_fn(checkmat)))
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE|LOCATION_ONFIELD)
    let rg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE | LOCATION_ONFIELD), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)
        duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
        // sg:Sub(rg)
        group::remove_card(sg, &rg);
    }
    true
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,0,PLAYER_EITHER,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 0, PLAYER_EITHER as u8, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(86120751) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_code(d, c, &[86120751]) && card::is_able_to_hand(d, c, None)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return e:GetHandler():IsAbleToDeck()
    //     and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_deck(d, h) && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and c:IsLocation(LOCATION_DECK) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_DECK)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc.nil()), None, REASON_EFFECT, None);
    }
}
