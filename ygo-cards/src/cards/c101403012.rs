//! Sinful Spoils of the White Crow Forest
use super::prelude::*;

pub const ID: u32 = 101403012;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by placing 1 Level 1 FIRE Monster Card from your face-up field or GY on the bottom of the Deck. You can only Special Summon "Sinful Spoils of the White Crow Forest" once per turn this way
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // When your opponent activates a card or effect (Quick Effect): You can return this card to the hand, then target 1 monster on the field (face-up) or in either GY; place it face-up in its owner's Spell & Trap Zone as a Continuous Spell. You can only use this effect of "Sinful Spoils of the White Crow Forest" once per turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    effect::set_cost(d, e2, Some(cost::self_to_hand()));
    effect::set_target(d, e2, Some(cost_target(pltg)));
    effect::set_operation(d, e2, Some(operation(plop)));
    card::register_effect(d, c, e2, false);
}
// s.spcostfilter(c,tp)
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevel(1) and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsMonsterCard() and c:IsFaceup()
    //   and c:IsAbleToDeckAsCost() and Duel.GetMZoneCount(tp,c)>0
    card::is_level(d, c, &[1])
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_monster_card(d, c)
        && card::is_faceup(d, c)
        && card::is_able_to_deck_as_cost(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// s.spcon(e,c): function(e,c) with c the summoned card (nil when the core only checks)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil,tp)
    duel::is_existing_matching_card(d, Some(&|d, c| spcostfilter(d, c, tp)), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c): the summoning procedure's target
fn sptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local g=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spcostfilter(d, c, tp)), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,1,nil,1,tp,HINTMSG_TODECK,nil,nil,true)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TODECK as u64), None, None, true)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
// s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local sg=e:GetLabelObject()
    // if sg then
    if let Some(sg) = effect::get_label_object(d, e).as_group().cloned() {
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_COST)
        duel::send_to_deck(d, sg, None, SEQ_DECKBOTTOM as i32, REASON_COST, None);
    }
}
// s.plfilter(c)
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // if not (c:IsMonster() and c:IsFaceup()) then return false end
    if !(card::is_monster(d, c) && card::is_faceup(d, c)) {
        return false;
    }
    // local owner=c:GetOwner()
    let owner = card::get_owner(d, c);
    // return Duel.GetLocationCount(owner,LOCATION_SZONE)>0 and c:CheckUniqueOnField(owner) and not c:IsForbidden()
    duel::get_location_count(d, owner, LOCATION_SZONE as u8, None, None, None).0 > 0
        && card::check_unique_on_field(d, c, owner, None, None)
        && !card::is_forbidden(d, c)
}
// local LOCATIONS_MZONE_GRAVE=LOCATION_MZONE|LOCATION_GRAVE
const LOCATIONS_MZONE_GRAVE: u32 = LOCATION_MZONE | LOCATION_GRAVE;
// s.pltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATIONS_MZONE_GRAVE) and s.plfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATIONS_MZONE_GRAVE) && plfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.plfilter,tp,LOCATIONS_MZONE_GRAVE,LOCATIONS_MZONE_GRAVE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&plfilter), tp, LOCATIONS_MZONE_GRAVE, LOCATIONS_MZONE_GRAVE, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.plfilter,tp,LOCATIONS_MZONE_GRAVE,LOCATIONS_MZONE_GRAVE,1,1,nil):GetFirst()
    let g = duel::select_target(d, tp, Some(&plfilter), tp, LOCATIONS_MZONE_GRAVE, LOCATIONS_MZONE_GRAVE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // if tc:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, tc, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, tp, 0);
    }
    true
}
// s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, tc, e) && !card::is_immune_to_effect(d, tc, e)) {
        return;
    }
    let owner = card::get_owner(d, tc);
    // if Duel.GetLocationCount(tc:GetOwner(),LOCATION_SZONE)==0 then
    if duel::get_location_count(d, owner, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        // Duel.SendtoGrave(tc,REASON_RULE,nil,PLAYER_NONE)
        duel::send_to_grave(d, CardSet::from_card(tc), REASON_RULE, None, Some(PLAYER_NONE as u8));
    } else {
        // elseif Duel.MoveToField(tc,tp,tc:GetOwner(),LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
        let is_monster = card::is_monster_card(d, tc);
        if duel::move_to_field(d, tc, tp, owner, LOCATION_SZONE as u16, POS_FACEUP as u8, is_monster, None) {
            // Treated as a Continuous Spell
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
