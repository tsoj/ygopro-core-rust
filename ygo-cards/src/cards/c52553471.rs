//! Over Fusion
use super::prelude::*;

pub const ID: u32 = 52553471;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    listed_series: &[SET_ELEMENTAL_HERO as u32, SET_NEO_SPACIAN as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp)`: `c:IsFaceup() and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA,0,1,nil,c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, c, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.cfilter(c,tc,tp)`
fn cfilter(d: &mut Duel, c: CardId, tc: CardId, tp: u8) -> bool {
    // if c:IsCode(tc:GetCode(nil,SUMMON_TYPE_FUSION,tp)) then return false end
    let codes = card::get_code_ctx(d, tc, None, SUMMON_TYPE_FUSION as u64, Some(tp));
    if card::is_code(d, c, &codes) {
        return false;
    }
    // return c:IsMonster() and (c:IsSetCard({SET_ELEMENTAL_HERO,SET_NEO_SPACIAN}) or c:IsLevel(10)) and not c:IsPublic()
    card::is_monster(d, c)
        && (card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16, SET_NEO_SPACIAN as u16]) || card::is_level(d, c, &[10]))
        && !card::is_public(d, c)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local cc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,tc,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, tc, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not cc then return end
    let cc = match group::get_first(d, &g) {
        Some(cc) => cc,
        None => return,
    };
    // Duel.ConfirmCards(1-tp,cc)
    duel::confirm_cards(d, 1 - tp, cc);
    // local code1,code2=cc:GetOriginalCodeRule()
    let codes = card::get_original_code_rule(d, cc);
    let code1 = codes.first().copied().nil();
    let code2 = codes.get(1).copied();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ADD_CODE)
    effect::set_code(d, e1, EFFECT_ADD_CODE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(code1)
    effect::set_value(d, e1, code1 as i64);
    // e1:SetOperation(s.chngcon)
    effect::set_operation(d, e1, Some(lua_fn(chngcon)));
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // if code2 then
    if let Some(code2) = code2 {
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetValue(code2)
        effect::set_value(d, e2, code2 as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_NEOS),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_NEOS]))), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX) {
        // Duel.SendtoGrave(cc,REASON_EFFECT)
        duel::send_to_grave(d, cc, REASON_EFFECT, None, None);
    } else if card::is_location(d, cc, LOCATION_HAND) {
        // Duel.SendtoDeck(cc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, cc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
    // if cc:IsLocation(LOCATION_HAND) then Duel.ShuffleHand(tp)
    if card::is_location(d, cc, LOCATION_HAND) {
        duel::shuffle_hand(d, tp);
    // elseif cc:IsLocation(LOCATION_DECK) then Duel.ShuffleDeck(tp)
    } else if card::is_location(d, cc, LOCATION_DECK) {
        duel::shuffle_deck(d, tp);
    // elseif cc:IsLocation(LOCATION_EXTRA) then Duel.ShuffleExtra(tp) end
    } else if card::is_location(d, cc, LOCATION_EXTRA) {
        duel::shuffle_extra(d, tp);
    }
}
/// `s.chngcon(scard,sumtype,tp)`: `(sumtype&MATERIAL_FUSION)~=0`
fn chngcon(_d: &mut Duel, a: Args) -> bool {
    (a.int(1) & MATERIAL_FUSION as i64) != 0
}
