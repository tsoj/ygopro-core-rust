//! Yakusa, Lord of the Eight Thunders
use super::prelude::*;

pub const ID: u32 = 90583279;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 Level/Rank 8 monster and apply an effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW + CATEGORY_ATKCHANGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsLevel(8) or c:IsRank(8)) and c:IsAbleToRemoveAsCost() and Duel.GetMZoneCount(tp,c)>0
    //   and aux.SpElimFilter(c,true)
    (card::is_level(d, c, &[8]) || card::is_rank(d, c, &[8]))
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp)
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,true,nil,tp)
    let rg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, true, NO_EX);
    // if rg and #rg>0 then
    if let Some(rg) = rg {
        if !rg.is_empty() {
            // e:SetLabelObject(rg)
            effect::set_label_object(d, e, Value::Group(rg));
            // return true
            return true;
        }
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local rg=e:GetLabelObject()
    // if not rg then return end
    let Some(rg) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
}
/// `s.tdfilter(c,tp)`
fn tdfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsLevel(8) or c:IsRank(8)) and (c:IsAbleToDeck() or c:IsAbleToExtra())
    //   and ((c:IsType(TYPE_RITUAL|TYPE_FUSION) and Duel.IsPlayerCanDraw(tp,1)) or c:IsType(TYPE_SYNCHRO|TYPE_XYZ))
    (card::is_level(d, c, &[8]) || card::is_rank(d, c, &[8]))
        && (card::is_able_to_deck(d, c) || card::is_able_to_extra(d, c))
        && ((card::is_type(d, c, TYPE_RITUAL | TYPE_FUSION) && duel::is_player_can_draw(d, tp, 1)) || card::is_type(d, c, TYPE_SYNCHRO | TYPE_XYZ))
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.tdfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && tdfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tdfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local tc=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&|d, c| tdfilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tc,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, tc.map(CardSet::from_card), 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // if tc:IsLocation(LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if card::is_location(d, tc, LOCATION_DECK) {
        duel::shuffle_deck(d, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc:IsType(TYPE_RITUAL|TYPE_FUSION) and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
    if card::is_type(d, tc, TYPE_RITUAL | TYPE_FUSION) && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
    // elseif tc:IsType(TYPE_SYNCHRO|TYPE_XYZ) and tc:IsLocation(LOCATION_EXTRA)
    //   and c:IsFaceup() and c:IsRelateToEffect(e) then
    else if card::is_type(d, tc, TYPE_SYNCHRO | TYPE_XYZ) && card::is_location(d, tc, LOCATION_EXTRA) && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Increase ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
