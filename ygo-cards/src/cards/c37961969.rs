//! Tearlaments Havnis
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 37961969;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon
    // local fusparams = {matfilter=Card.IsAbleToDeck,extrafil=s.extramat,extraop=s.extraop,gc=Fusion.ForcedHandler,extratg=s.extratarget}
    let fusparams = fusion::SummonEff {
        matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c))),
        extrafil: Some(Arc::new(extramat)),
        extraop: Some(extraop()),
        gc: Some(fusion::forced_handler()),
        extratg: Some(cost_target(extratarget)),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return Duel.GetCurrentPhase()~=PHASE_DAMAGE and e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(Fusion.SummonEffTG(fusparams))
    let tg = fusion::summon_eff_tg(d, fusparams.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusparams))
    let op = fusion::summon_eff_op(d, fusparams);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and re:GetActivateLocation()==LOCATION_MZONE
    let re = a.re.nil();
    a.rp == 1 - a.tp && effect::is_monster_effect(d, re) && effect::get_activate_location(d, re) == LOCATION_MZONE
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsPlayerCanDiscardDeck(tp,3) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_player_can_discard_deck(d, a.tp, 3);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 3);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.DiscardDeck(tp,3,REASON_EFFECT)
        duel::discard_deck(d, tp, 3, REASON_EFFECT);
    }
}
/// `function(e) return Duel.GetCurrentPhase()~=PHASE_DAMAGE and e:GetHandler():IsReason(REASON_EFFECT) end` (e2)
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_current_phase(d) != PHASE_DAMAGE && {
        let h = effect::get_handler(d, a.e).nil();
        card::is_reason(d, h, REASON_EFFECT)
    }
}
/// `s.extramat(e,tp,mg)`
fn extramat(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_GRAVE,0,nil)
    (Some(duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_GRAVE, 0, NO_EX)), None)
}
/// `s.extratarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop() -> LuaFn {
    fusion::extra_op(|d, _e, _tc, tp, sg| {
        // local gg=sg:Filter(Card.IsLocation,nil,LOCATION_HAND|LOCATION_GRAVE)
        let gg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE), NO_EX);
        // if #gg>0 then Duel.HintSelection(gg,true) end
        if !gg.is_empty() {
            duel::hint_selection(d, &gg, true);
        }
        // local rg=sg:Filter(Card.IsFacedown,nil)
        let rg = group::filter(d, sg, &|d, c| card::is_facedown(d, c), NO_EX);
        // if #rg>0 then Duel.ConfirmCards(1-tp,rg) end
        if !rg.is_empty() {
            duel::confirm_cards(d, 1 - tp, &rg);
        }
        // Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)
        duel::send_to_deck(d, sg.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None);
        // local dg=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_DECK)
        let op = duel::get_operated_group(d);
        let dg = group::filter(d, &op, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        // local ct=dg:FilterCount(Card.IsControler,nil,tp)
        let ct = group::filter_count(d, &dg, &|d, c| card::is_controler(d, c, tp), NO_EX);
        // if ct>0 then Duel.SortDeckbottom(tp,tp,ct) end
        if ct > 0 {
            duel::sort_deckbottom(d, tp, tp, ct as u16);
        }
        // if #dg>ct then Duel.SortDeckbottom(tp,1-tp,#dg-ct) end
        if dg.len() > ct {
            duel::sort_deckbottom(d, tp, 1 - tp, (dg.len() - ct) as u16);
        }
        // sg:Clear()
        sg.clear();
        true
    })
}
