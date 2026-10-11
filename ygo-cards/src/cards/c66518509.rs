//! Mementotlan Fusion
use super::prelude::*;

pub const ID: u32 = 66518509;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEMENTO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Fusion Summon 1 Fusion Monster, including 1 "Memento" monster as material
    // local e1=Fusion.CreateSummonEff({handler=c,extrafil=s.fextra,extratg=s.extratg,extraop=s.extraop})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            extrafil: Some(std::sync::Arc::new(fextra)),
            extratg: Some(cost_target(extratg)),
            extraop: Some(fusion::extra_op(extraop)),
            ..Default::default()
        },
    );
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Destroy 1 "Memento" monster you control and search 1 "Memento" Spell/Trap
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Register destruction of monsters
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROY)
        effect::set_code(d, ge1, EVENT_DESTROY);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkfilter(c)`
fn checkfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsPreviousLocation(LOCATION_MZONE) or (c:IsMonster() and not c:IsPreviousLocation(LOCATION_ONFIELD))) and c:IsReason(REASON_EFFECT)
    (card::is_previous_location(d, c, LOCATION_MZONE) || (card::is_monster(d, c) && !card::is_previous_location(d, c, LOCATION_ONFIELD)))
        && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.checkfilter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &checkfilter, NO_EX);
    // if #g==0 then return end
    if g.len() == 0 {
        return;
    }
    // for p=0,1 do
    for p in 0..2u8 {
        // if g:IsExists(Card.IsPreviousControler,1,nil,p) and not Duel.HasFlagEffect(p,id) then
        if group::is_exists(d, &g, &|d, c| card::is_previous_controler(d, c, p), 1, NO_EX) && !duel::has_flag_effect(d, p, ID, None) {
            // Duel.RegisterFlagEffect(p,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.extramatfilter(c)`
fn extramatfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MEMENTO) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_MEMENTO as u16]) && card::is_able_to_deck(d, c)
}
/// `s.checkmat(tp,sg,fc)`
fn checkmat(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_MEMENTO)
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_set_card(d, c, &[SET_MEMENTO as u16]), 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // local sg=nil
    let mut sg = None;
    // if Duel.HasFlagEffect(tp,id) then
    if duel::has_flag_effect(d, tp, ID, None) {
        // sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.extramatfilter),tp,LOCATION_GRAVE,0,nil)
        sg = Some(duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| extramatfilter(d, c))), tp, LOCATION_GRAVE, 0, NO_EX));
    }
    // return sg,s.checkmat
    (sg, Some(lua_fn(checkmat)))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let rg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #rg>0 then
    if rg.len() > 0 {
        // Duel.HintSelection(rg,true)
        duel::hint_selection(d, &rg, true);
        // Duel.SendtoDeck(rg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT|REASON_MATERIAL|REASON_FUSION)
        duel::send_to_deck(d, rg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None);
        // sg:RemoveCard(rg)
        group::remove_card(sg, &rg);
    }
    true
}
/// `function(e) return Duel.IsMainPhase() end`
fn e1_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MEMENTO) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MEMENTO as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return g.len() > 0 && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local desg=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,0,1,1,nil)
    let desg = duel::select_matching_card(d, tp, None, tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if Duel.Destroy(desg,REASON_EFFECT)==0 then return end
    if duel::destroy(d, desg, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local hg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let hg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #hg>0 then
    if hg.len() > 0 {
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg);
    }
}
