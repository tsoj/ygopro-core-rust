//! Tearlaments Merrli
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 74078255;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Mill 3 cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
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
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e) return Duel.GetCurrentPhase()~=PHASE_DAMAGE and e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(Fusion.SummonEffTG(fusparams))
    let tg = fusion::summon_eff_tg(d, fusparams.clone());
    effect::set_target(d, e3, Some(tg));
    // e3:SetOperation(Fusion.SummonEffOP(fusparams))
    let op = fusion::summon_eff_op(d, fusparams);
    effect::set_operation(d, e3, Some(op));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return Duel.GetCurrentPhase()~=PHASE_DAMAGE and e:GetHandler():IsReason(REASON_EFFECT) end` (e3)
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentPhase()~=PHASE_DAMAGE and e:GetHandler():IsReason(REASON_EFFECT)
    duel::get_current_phase(d) != PHASE_DAMAGE && {
        let h = effect::get_handler(d, a.e).nil();
        card::is_reason(d, h, REASON_EFFECT)
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,3) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 3);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 3);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,3,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 3, REASON_EFFECT);
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
