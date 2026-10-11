//! Clown Crew New Face
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 6547248;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // During the Standby Phase: You can shuffle 3 "Clown Crew" cards from your GY and/or banishment into the Deck, then draw 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsStandbyPhase() end)
    effect::set_condition(d, e1, Some(lua_fn(|d, _a: Args| duel::is_standby_phase(d, None))));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // e1:SetHintTiming(TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you Tribute Summon a monster: You can target other Spells/Traps on the field, up to the number of monsters Tributed for its Tribute Summon; destroy them
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is sent to the GY, except from the field: You can place this card face-up on your field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e) return not e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(
        d,
        e3,
        Some(lua_fn(|d, a: Args| {
            let h = effect::get_handler(d, a.e(0)).nil();
            !card::is_previous_location(d, h, LOCATION_ONFIELD)
        })),
    );
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp)
    //     and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 0)
            && duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 3, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,3,nil) then return end
    let filter = aux::necro_valley_filter(tdfilter);
    if !duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 3, NO_EX) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,3,3,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 3, 3, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
        if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, a.tp);
        }
        // if Duel.IsPlayerCanDraw(tp) then
        if duel::is_player_can_draw(d, a.tp, 0) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg).nil();
    // if not (ec:IsSummonPlayer(tp) and ec:IsTributeSummoned()) then return false end
    if !(card::is_summon_player(d, ec, a.tp) && card::is_tribute_summoned(d, ec)) {
        return false;
    }
    // local mat_count=ec:GetMaterialCount()
    let mat_count = card::get_material_count(d, ec) as i64;
    // if mat_count>e:GetLabel() then
    if mat_count > effect::get_label(d, a.e) {
        // e:SetLabel(mat_count)
        effect::set_label(d, a.e, &[mat_count]);
    }
    // return mat_count>0
    mat_count > 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc) && chkc != c;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local mat_count=e:GetLabel()
        let mat_count = effect::get_label(d, a.e);
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1,mat_count)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, mat_count, 0);
        // return Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
        return duel::is_existing_target(d, Some(&|d, x| card::is_spell_trap(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local max_count=c:GetFlagEffectLabel(id)
    let max_count = card::get_flag_effect_label(d, c, ID).first().copied().nil();
    // c:ResetFlagEffect(id)
    card::reset_flag_effect(d, c, ID);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,max_count,c)
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_spell_trap(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, max_count as u16, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
